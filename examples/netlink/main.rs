use tll::channel::*;
use tll::config::Config;
use tll::logger::Logger;
use tll::processor::Loop;

use std::convert::TryInto;
use std::time::Duration;

mod netlink_scheme;
use crate::netlink_scheme::*;
mod udev_scheme;
mod timer_scheme;
use crate::timer_scheme::*;

#[derive(Debug, Default)]
struct SystemState {
    time: Duration,
    link: Option<String>,
    battery: i32,
    ac: bool,
}

enum TimerBind<'a> {
    RefAbsolute(&'a absolute),
    RefRelative(&'a relative),
    SizeError(i32),
    Unknown(i32),
}

fn timer_bind(m: &Message) -> TimerBind {
    match m.msgid() {
        absolute::MSGID => match absolute::bind(m.data()) {
            Some(m) => TimerBind::RefAbsolute(m),
            None => TimerBind::SizeError(m.msgid()),
        },
        relative::MSGID => match relative::bind(m.data()) {
            Some(m) => TimerBind::RefRelative(m),
            None => TimerBind::SizeError(m.msgid()),
        },
        _ => TimerBind::Unknown(m.msgid()),
    }
}

enum NetlinkBind<'a> {
    RefLink(&'a Link),
    RefRoute4(&'a Route4),
    RefRoute6(&'a Route6),
    SizeError(i32),
    Unknown(i32),
}

fn netlink_bind(m: &Message) -> NetlinkBind {
    match m.msgid() {
        Link::MSGID => match Link::bind(m.data()) {
            Some(m) => NetlinkBind::RefLink(m),
            None => NetlinkBind::SizeError(m.msgid()),
        },
        Route4::MSGID => match Route4::bind(m.data()) {
            Some(m) => NetlinkBind::RefRoute4(m),
            None => NetlinkBind::SizeError(m.msgid()),
        },
        Route6::MSGID => match Route6::bind(m.data()) {
            Some(m) => NetlinkBind::RefRoute6(m),
            None => NetlinkBind::SizeError(m.msgid()),
        },
        _ => NetlinkBind::Unknown(m.msgid()),
    }
}

impl SystemState {
    pub fn on_timer(&mut self, m: &Message) -> i32 {
        if m.get_type() != MsgType::Data {
            return 0;
        }
        match timer_bind(m) {
            TimerBind::RefAbsolute(msg) => {
                println!("Timer: {:?}", { msg.ts });
                if let Ok (ts) = TryInto::<Duration>::try_into(msg.ts.value) {
                    self.time = ts;
                } else {
                    println!("Negative timestamp")
                }
            }
            _ => {}
        }
        /*
        match m.data().try_into().map(|a| u64::from_ne_bytes(a)) {
            Ok(r) => self.time = Duration::from_nanos(r),
            _ => (),
        }
        */
        self.dump();
        0
    }

    pub fn on_route(&mut self, m: &Message) -> i32 {
        if m.get_type() != MsgType::Data {
            return 0;
        }
        match netlink_bind(m) {
            NetlinkBind::RefLink(msg) => {
                let name = unsafe { msg.name.as_str_unchecked() };
                println!("Link: {:?} {} {}", msg.action, name, msg.up);
                if msg.action == Action::New || msg.up == 1 {
                    return 0;
                }
                if self.link.as_ref().map(|s| s.as_str()) == Some(name) {
                    self.link = None;
                }
            }
            NetlinkBind::RefRoute4(r4) => {
                let name = unsafe { r4.oif.as_str_unchecked() };
                println!(
                    "Route4: {:?} {}/{} -> {}",
                    r4.action,
                    std::net::Ipv4Addr::from(u32::from_be(r4.dst)),
                    r4.dst_mask,
                    name
                );
                if r4.dst_mask != 0 {
                    return 0;
                }
                println!("Default route");
                match r4.action {
                    Action::New => self.link = Some(name.to_string()),
                    Action::Delete => self.link = None,
                    //_ => (),
                }
            }
            _ => (),
        }
        self.dump();
        0
    }

    pub fn on_power(&mut self, m: &Message) -> i32 {
        if m.get_type() != MsgType::Data {
            return 0;
        }
        if m.msgid != udev_scheme::Device::MSGID {
            return 0;
        }
        if let Some (data) = udev_scheme::Device::bind(m.data()) {
            match data.subsystem.as_str() {
                Ok("power_supply") => (),
                _ => return 0,
            }
            match data.sysname.as_str() {
                Ok("AC") => (),
                _ => return 0,
            }
            for p in unsafe { data.properties.data() } {
                match p.name.as_str() {
                    Ok("POWER_SUPPLY_ONLINE") =>
                        match p.value.as_str() {
                            Ok("0") => self.ac = false,
                            Ok("1") => self.ac = true,
                            _ => (),
                        },
                    _  => (),
                }
            }
        }
        self.dump();
        0
    }

    pub fn dump(&self) {
        println!("Time: {:?}, Link: {:?}, AC: {:?}", self.time, self.link, self.ac);
    }
}

enum TimerCallback {}
enum RouteCallback {}
enum PowerCallback {}

impl CallbackMut<TimerCallback> for SystemState {
    fn message_callback_mut(&mut self, _c: &Channel, m: &Message) -> i32 {
        self.on_timer(m)
    }
}

impl CallbackMut<RouteCallback> for SystemState {
    fn message_callback_mut(&mut self, _c: &Channel, m: &Message) -> i32 {
        self.on_route(m)
    }
}

impl CallbackMut<PowerCallback> for SystemState {
    fn message_callback_mut(&mut self, _c: &Channel, m: &Message) -> i32 {
        self.on_power(m)
    }
}

pub fn main() {
    let mut state = SystemState::default();

    let cfg = Config::load_data("yamls", "{type: spdlog}").unwrap();
    Logger::config(&cfg).unwrap();

    let ctx = Context::new();
    ctx.load(
        "/home/psha/src/tll-netlink/build/tll-netlink",
        "channel_module",
    )
    .expect("Failed to load module");
    ctx.load(
        "/home/psha/src/tll-udev/build/tll-udev",
        "channel_module",
    )
    .expect("Failed to load module");

    let mut netlink = ctx
        .channel("netlink://;name=netlink;dump=scheme")
        .expect("Failed to create channel");
    //netlink.callback_add(&|_, m| state.on_route(m), None).expect("Failed to add callback");
    netlink.callback_add_mut::<SystemState, RouteCallback>(&mut state, None)
        .expect("Failed to add callback");

    let mut udev = ctx
        .channel("udev://;name=udev;dump=scheme;subsystem=power_supply")
        .expect("Failed to create channel");
    netlink.callback_add_mut::<SystemState, PowerCallback>(&mut state, None)
        .expect("Failed to add callback");

    let mut tc = ctx
        .channel("timer://;interval=1s;clock=realtime;dump=frame;name=timer;skip-old=yes")
        .expect("Failed to create channel");

    //tc.callback_add_mut(&|_, m| state.on_timer(m), None).expect("Failed to add callback");
    tc.callback_add_mut::<SystemState, TimerCallback>(&mut state, None)
        .expect("Failed to add callback");

    let mut l = Loop::new("rust");
    l.add(&mut netlink).expect("Failed to add channel to loop");
    l.add(&mut udev).expect("Failed to add channel to loop");
    l.add(&mut tc).expect("Failed to add channel to loop");
    tc.open("").expect("Failed to open channel");
    netlink.open("").expect("Failed to open channel");
    udev.open("").expect("Failed to open channel");
    loop {
        l.step(1000).expect("Step failed");
    }
}
