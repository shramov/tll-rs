use tll::channel::*;

use tll::error::*;
use tll::config::{Config};
use tll::props::{Props};
use tll::channel::impl_::*;

fn callback(c: &Channel, m: &Message) -> i32
{
    println!("Callback: {} {:?} {:?}", c.name(), m.type_, m.msgid);
    0
}

#[ derive(Debug, Default) ]
struct Echo { internal: Internal }

impl ChannelImpl for Echo {
    fn new() -> Self { Echo { internal: Internal::new() } } // counter: 0 } }
    fn internal(&mut self) -> &mut Internal { &mut self.internal }

    fn init(&mut self, _url: &Config, parent: Option<Channel>, _: &Context) -> Result<()>
    {
        println!("Create channel, parent {:?}", parent);
        Ok(()) 
    }

    fn open(&mut self, url: &Props) -> Result<()>
    {
        self.internal.set_state(State::Opening);
        println!("Open channel {:?}", url);
        Ok(()) 
    }

    fn process(&mut self) -> Result<i32>
    {
        if self.internal.state() == State::Opening {
            self.internal.set_state(State::Active);
        }
        Ok(0)
    }

    fn post(&mut self, msg: &Message) -> Result<i32>
    {
        self.internal.callback_data(msg);
        Ok(0)
    }
}

//#[test]
tll::declare_channel_impl!(custom_impl, Echo, "echo");

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

        assert!(c.process().is_ok());
        assert_eq!(c.state(), State::Active);

        assert!(c.post(Message::new().set_msgid(100).set_seq(100).set_data(b"abcd")).is_ok())
    }

    Ok(())
}
