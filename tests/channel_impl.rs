use tll::channel::*;

use tll::error::*;
use tll::config::{Config};
use tll::channel::base::*;

fn callback(c: &Channel, m: &Message) -> i32
{
    println!("Callback: {} {:?} {:?}", c.name(), m.type_, m.msgid);
    0
}

#[ derive(Debug, Default) ]
struct Echo { base: Base }

impl Extension for Echo {
    type Inner = Base;

    fn inner(&self) -> &Self::Inner {
        &self.base
    }
    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.base
    }
}

impl ChannelImpl for Echo {
    fn channel_protocol() -> &'static str { "echo" }
    fn open_policy() -> OpenPolicy { OpenPolicy::Manual }

    fn init(&mut self, url: &Config, master: Option<Channel>, context: &Context) -> Result<()>
    {
        println!("Create channel, master {:?}", master);
        self.logger().info(&format!("Create channel, master {:?}", master));
        self.inner_mut().init(url, master, context)
    }

    fn open(&mut self, url: &Config) -> Result<()>
    {
        println!("Open channel");
        self.inner_mut().open(url)
    }

    fn process(&mut self) -> Result<i32>
    {
        println!("Called process");
        if self.state() == State::Opening {
            self.set_state(State::Active);
        }
        Ok(0)
    }

    fn post(&mut self, msg: &Message) -> Result<()>
    {
        self.base_mut().callback_data(msg);
        Ok(())
    }
}

//#[test]
tll::declare_channel_impl!(custom_impl, Echo);

#[test]
fn test() -> Result<()> {
    let ctx = Context::new();
    assert!(ctx.channel("echo://;name=custom").is_err());
    ctx.register(custom_impl())?;

    {
        let mut r = ctx.channel("echo://host;name=custom");
        assert!(r.is_ok());
        println!("Created channel");
        let c = r.as_mut()?.get_mut();

        assert!(c.callback_add(&callback, None).is_ok());

        assert_eq!(c.name(), "custom");
        assert_eq!(c.state(), State::Closed);

        assert!(c.open("").is_ok());
        assert_eq!(c.state(), State::Opening);

        assert_eq!(c.process(), Ok(0));
        assert_eq!(c.state(), State::Active);

        assert!(c.post(Message::new().set_msgid(100).set_seq(100).set_data(b"abcd")).is_ok())
    }

    Ok(())
}
