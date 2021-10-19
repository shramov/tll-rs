use tll::channel::*;
use tll::processor::Loop;
use tll::config::Config;
use tll::logger::Logger;

use std::convert::TryInto;
use std::time::Duration;

mod netlink_scheme;
use crate::netlink_scheme::*;

#[derive (Debug, Default)]
struct SystemState
{
    time: Duration,
    link: Option<String>,
}

impl SystemState
{
    pub fn timer_cb(&mut self, m: &Message) -> i32
    {
        if m.get_type() != MsgType::Data { return 0; }
        match m.data().try_into().map(|a| u64::from_ne_bytes(a)) {
            Ok(r) => self.time = Duration::from_nanos(r),
            _ => (),
        }
        self.dump();
        0
    }

    pub fn route_cb(&mut self, m: &Message) -> i32
    {
        if m.get_type() != MsgType::Data { return 0; }
        match m.msgid() {
            Link::MSGID => {
                let msg = unsafe { &*(m.data().as_ptr() as *const Link) };
                let name = unsafe { msg.name.as_str_unchecked() };
                println!("Link: {:?} {} {}", msg.action, name, msg.up);
                if msg.action == Action::New || msg.up == 1 { return 0; }
                if self.link.as_ref().map(|s| s.as_str()) == Some(name) {
                    self.link = None;
                }
            },
            Route4::MSGID => {
                let r4 = unsafe { &*(m.data().as_ptr() as *const Route4) };
                let name = unsafe { r4.oif.as_str_unchecked() };
                println!("Route4: {:?} {}/{} -> {}", r4.action, std::net::Ipv4Addr::from(u32::from_be(r4.dst)), r4.dst_mask, name);
                if r4.dst_mask != 0 { return 0; }
                println!("Default route");
                match r4.action {
                    Action::New => self.link = Some(name.to_string()),
                    Action::Delete => self.link = None,
                    //_ => (),
                }
            },
            _ => (),
        }
        self.dump();
        0
    }

    pub fn dump(&self)
    {
        println!("Time: {:?}, Link: {:?}", self.time, self.link);
    }
}

pub fn main()
{
    let mut state = SystemState::default();

    let cfg = Config::load_data("yamls", "{type: spdlog}").unwrap();
    Logger::config(&cfg).unwrap();

    let ctx = Context::new();
    ctx.load("/home/psha/src/tll-netlink/build/tll-netlink", "channel_module").expect("Failed to load module");

    let mut c = ctx.channel("netlink://;name=netlink;dump=scheme").expect("Failed to create channel");
    c.callback_add(&|_, m| state.route_cb(m), None).expect("Failed to add callback");

    let mut tc = ctx.channel("timer://;interval=1s;clock=realtime;dump=yes;name=timer").expect("Failed to create channel");

    tc.callback_add(&|_, m| state.timer_cb(m), None).expect("Failed to add callback");

    let mut l = Loop::new("rust");
    l.add(&mut c).expect("Failed to add channel to loop");
    l.add(&mut tc).expect("Failed to add channel to loop");
    //tc.open("").expect("Failed to open channel");
    c.open("").expect("Failed to open channel");
    loop {
        l.step(1000).expect("Step failed");
    }
}
