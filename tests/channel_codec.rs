use tll::channel::codec::*;
use tll::channel::*;

use tll::error::*;
//use tll::config::{Config};
//use tll::props::{Props};
use tll::channel::impl_::*;

#[derive(Debug, Default)]
struct Xor {
    encbuf: Vec<u8>,
    decbuf: Vec<u8>,
}

type XorCodec = Codec<Xor>;

impl Xor {
    fn convert<'a>(data: &[u8], vec: &'a mut Vec<u8>) -> &'a [u8] {
        if vec.capacity() < data.len() {
            vec.reserve(data.len() - vec.len());
        }
        vec.truncate(0);
        for i in data {
            vec.push(i ^ 0x01u8);
        }
        vec
    }
}

impl CodecImpl for Xor {
    fn channel_protocol() -> &'static str { "xor+" }

    fn encode(&mut self, msg: &Message) -> Result<Message> {
        let mut m = Message::new();
        m.set_data(Xor::convert(msg.data(), &mut self.encbuf));
        Ok(m)
    }

    fn decode(&mut self, msg: &Message) -> Result<Message> {
        let mut m = Message::new();
        m.set_data(Xor::convert(msg.data(), &mut self.decbuf));
        Ok(m)
    }
}

tll::declare_channel_impl!(custom_impl, XorCodec);

#[test]
fn test() -> Result<()> {
    let ctx = Context::new();
    ctx.register(custom_impl())?;

    let mut c = ctx.channel("xor+null://host;name=custom;dump=text+hex").unwrap();
    println!("Created channel");

    //assert!(c.callback_add(&callback, None).is_ok());

    assert_eq!(c.name(), "custom");
    assert_eq!(c.state(), State::Closed);

    assert!(c.open("").is_ok());
    assert_eq!(c.state(), State::Active);

    assert!(c
        .post(Message::new().set_msgid(100).set_seq(100).set_data(b"abcd"))
        .is_ok());

    Ok(())
}
