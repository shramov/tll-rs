use tll::channel::*;
use tll::config::Config;
use tll::logger::Logger;
use tll::processor::Loop;
use tll::result::EINVAL;

use chrono::{DateTime, Local};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

mod netlink_scheme;
use crate::netlink_scheme::*;
mod nl80211_scheme;
mod timer_scheme;
mod udev_scheme;
use crate::timer_scheme::*;

#[derive(Debug)]
struct SystemState {
    time: DateTime<Local>,
    link: Option<String>,
    ssid: Option<String>,
    battery_file: File,
    battery_buf: [u8; 16],
    battery: u8, // Percentage
    ac: bool,
}

impl Default for SystemState {
    fn default() -> Self {
        SystemState {
            time: Local::now(),
            link: None,
            ssid: None,
            battery: 0,
            ac: false,
            battery_buf: [0; 16],
            battery_file: File::open("/sys/class/power_supply/BAT0/capacity").unwrap(),
        }
    }
}

enum TimerBind<Buf: MemRead> {
    RefAbsolute(absolute<Buf>),
    RefRelative(relative<Buf>),
    SizeError(i32),
    Unknown(i32),
}

fn timer_bind(m: &Message) -> TimerBind<&'_ [u8]> {
    match m.msgid() {
        absolute::<&[u8]>::MSGID => match absolute::bind(m.data()) {
            Ok(m) => TimerBind::RefAbsolute(m),
            _ => TimerBind::SizeError(m.msgid()),
        },
        relative::<&[u8]>::MSGID => match relative::bind(m.data()) {
            Ok(m) => TimerBind::RefRelative(m),
            _ => TimerBind::SizeError(m.msgid()),
        },
        _ => TimerBind::Unknown(m.msgid()),
    }
}

enum NetlinkBind<Buf: MemRead> {
    RefLink(Link<Buf>),
    RefRoute4(Route4<Buf>),
    RefRoute6(Route6<Buf>),
    SizeError(i32),
    Unknown(i32),
}

fn netlink_bind(m: &Message) -> NetlinkBind<&'_ [u8]> {
    match m.msgid() {
        Link::<&[u8]>::MSGID => match Link::bind(m.data()) {
            Ok(m) => NetlinkBind::RefLink(m),
            _ => NetlinkBind::SizeError(m.msgid()),
        },
        Route4::<&[u8]>::MSGID => match Route4::bind(m.data()) {
            Ok(m) => NetlinkBind::RefRoute4(m),
            _ => NetlinkBind::SizeError(m.msgid()),
        },
        Route6::<&[u8]>::MSGID => match Route6::bind(m.data()) {
            Ok(m) => NetlinkBind::RefRoute6(m),
            _ => NetlinkBind::SizeError(m.msgid()),
        },
        _ => NetlinkBind::Unknown(m.msgid()),
    }
}

impl SystemState {
    fn update_battery(&mut self) -> tll::result::Result<()> {
        self.battery_file.seek(SeekFrom::Start(0))?;
        let size = self.battery_file.read(&mut self.battery_buf)?;
        let string = std::str::from_utf8(&self.battery_buf[0..size])?.trim();
        self.battery = u8::from_str_radix(string, 10).map_err(|x| format!("Failed to convert '{string}': {x}"))?;
        Ok(())
    }

    pub fn on_timer(&mut self, m: &Message) -> tll::result::Result<()> {
        if m.get_type() != MsgType::Data {
            return Ok(());
        }
        match timer_bind(m) {
            TimerBind::RefAbsolute(msg) => {
                self.time = msg.get_ts().as_local_datetime()?;
            }
            _ => {}
        }
        self.update_battery()?;
        self.dump();
        Ok(())
    }

    pub fn on_route(&mut self, m: &Message) -> Result<(), tll::Error> {
        if m.get_type() != MsgType::Data {
            return Ok(());
        }
        match netlink_bind(m) {
            NetlinkBind::RefLink(msg) => {
                let name = msg.get_name()?;
                //println!("Link: {:?} {} {}", msg.action, name, msg.up);
                if msg.get_up() == 1 {
                    return Ok(());
                }
                if self.link.as_ref().map(|s| s.as_str()) == Some(name) {
                    self.link = None;
                }
            }
            NetlinkBind::RefRoute4(r4) => {
                let name = r4.get_oif()?;
                /*
                println!(
                    "Route4: {:?} {}/{} -> {}",
                    r4.action,
                    std::net::Ipv4Addr::from(u32::from_be(r4.dst)),
                    r4.dst_mask,
                    name
                );
                */
                if r4.get_dst_mask() != 0 {
                    return Ok(());
                }
                //println!("Default route");
                match r4.get_action() {
                    Action::New => self.link = Some(name.to_string()),
                    Action::Delete => self.link = None,
                    //_ => (),
                }
            }
            _ => (),
        }
        //self.dump();
        Ok(())
    }

    pub fn on_nl80211(&mut self, m: &Message) -> i32 {
        if m.get_type() != MsgType::Data {
            return 0;
        }
        if m.msgid != nl80211_scheme::Interface::<&[u8]>::MSGID {
            return 0;
        }
        if let Ok(data) = nl80211_scheme::Interface::bind(m.data()) {
            match data.get_ssid() {
                Ok("") => self.ssid = None,
                Ok(ssid) => self.ssid = Some(ssid.into()),
                Err(_) => self.ssid = None,
            }
        }
        //self.dump();
        0
    }

    pub fn on_power(&mut self, m: &Message) -> Result<(), tll::Error> {
        if m.get_type() != MsgType::Data {
            return Ok(());
        }
        if m.msgid != udev_scheme::Device::<&[u8]>::MSGID {
            return Ok(());
        }
        if let Ok(data) = udev_scheme::Device::<&[u8]>::bind(m.data()) {
            match data.get_subsystem() {
                Ok("power_supply") => (),
                _ => return Ok(()),
            }
            match data.get_sysname() {
                Ok("AC") => (),
                _ => return Ok(()),
            }
            for p in data.get_properties()?.iter() {
                match p.get_name() {
                    Ok("POWER_SUPPLY_ONLINE") => match p.get_value() {
                        Ok("0") => self.ac = false,
                        Ok("1") => self.ac = true,
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
        //self.dump();
        Ok(())
    }

    pub fn dump(&self) {
        let link: &str = self.ssid.as_ref().or(self.link.as_ref()).map(String::as_str).unwrap_or("-");
        let ac_sym = if self.ac { "🗲" } else { "" };
        println!(
            "{} {} {}{:2}%",
            self.time.format("%Y-%m-%d %H:%M:%S"),
            link,
            ac_sym,
            self.battery
        );
    }
}

enum TimerCallback {}
enum RouteCallback {}
enum NL80211Callback {}
enum PowerCallback {}

impl CallbackMut<TimerCallback> for SystemState {
    fn message_callback_mut(&mut self, _c: &Channel, m: &Message) -> i32 {
        match self.on_timer(m) {
            Ok(_) => 0,
            Err(e) => {
                println!("Timer callback failed: {e}");
                EINVAL
            }
        }
    }
}

impl CallbackMut<RouteCallback> for SystemState {
    fn message_callback_mut(&mut self, _c: &Channel, m: &Message) -> i32 {
        let _ = self.on_route(m);
        0
    }
}

impl CallbackMut<NL80211Callback> for SystemState {
    fn message_callback_mut(&mut self, _c: &Channel, m: &Message) -> i32 {
        self.on_nl80211(m)
    }
}

impl CallbackMut<PowerCallback> for SystemState {
    fn message_callback_mut(&mut self, _c: &Channel, m: &Message) -> i32 {
        let _ = self.on_power(m);
        0
    }
}

pub fn main() -> tll::result::Result<()> {
    let mut state = SystemState::default();

    let cfg = Config::load_data("yamls", "{type: spdlog, levels.tll: error}")?;
    Logger::config(&cfg)?;

    let ctx = Context::new();
    ctx.load("/home/psha/src/tll-netlink/build/tll-netlink")?;
    ctx.load("/home/psha/src/tll-udev/build/tll-udev")?;

    let mut netlink = ctx.channel("netlink://;name=netlink;dump=scheme;addr=no;neigh=no")?;
    //netlink.callback_add(&|_, m| state.on_route(m), None).expect("Failed to add callback");
    netlink.callback_add_mut::<SystemState, RouteCallback>(&mut state, None)?;

    let mut nl80211 = ctx.channel("nl80211://;name=nl80211;dump=scheme;addr=no;neigh=no")?;
    nl80211.callback_add_mut::<SystemState, NL80211Callback>(&mut state, None)?;

    let mut udev = ctx.channel("udev://;name=udev;dump=scheme;subsystem=power_supply")?;
    udev.callback_add_mut::<SystemState, PowerCallback>(&mut state, None)?;

    let mut tc = ctx.channel("timer://;interval=1s;clock=realtime;dump=frame;name=timer;skip-old=yes")?;

    //tc.callback_add_mut(&|_, m| state.on_timer(m), None).expect("Failed to add callback");
    tc.callback_add_mut::<SystemState, TimerCallback>(&mut state, None)?;

    let mut l = Loop::new("rust")?;
    l.add(&mut netlink)?;
    l.add(&mut nl80211)?;
    l.add(&mut udev)?;
    l.add(&mut tc)?;
    tc.open(None)?;
    netlink.open(None)?;
    nl80211.open(None)?;
    udev.open(None)?;
    loop {
        l.step(1000)?;
    }
}
