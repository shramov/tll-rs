use tll::channel::*;
use tll::config::*;
use tll::error::*;

mod scheme_scheme;
use crate::scheme_scheme::*;
//use crate::scheme_scheme::SCHEME_STRING;

#[allow(dead_code)]
fn check(m: &Message) -> Result<()>
{
    if m.get_type() != MsgType::Data { return Ok(()); }
    println!("Callback: {:?} {:?}", m.get_type(), m.msgid);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let ctx = Context::new();

    let url = Config::load_data("yamls", &format!("
tll.proto: yaml
name: yaml
dump: scheme
config:
  - name: msg
    seq: 1
    data:
        i8: -1
        u8: 1
        i16: -1000
        u16: 1000
        i32: -1000000
        u32: 1000000
        i64: -1000000000
        f64: 1.234
        d128: 1234567890.e-5
        b16: bytes
        c16: string
scheme: {}
", SCHEME_STRING)).ok_or("Failed to load config")?;

    let mut c = ctx.channel_url(&url)?;
    let mut r = Err(Error::from("No message received"));
    let check = |m : &Message| -> Result<()> {
        assert_eq!(m.msgid(), msg::MSGID);
        let data = msg::bind(m.data()).ok_or("Failed to bind")?;
        assert_eq!({ data.i8 }, -1);
        assert_eq!({ data.u8 }, 1);
        assert_eq!({ data.i16 }, -1000);
        assert_eq!({ data.u16 }, 1000);
        assert_eq!({ data.i32 }, -1000000);
        assert_eq!({ data.u32 }, 1000000);
        assert_eq!({ data.i64 }, -1000000000);
        assert_eq!({ data.f64 }, 1.234);
        assert_eq!(format!("{}", {data.d128}), "1234567890.E-5");
        assert_eq!(data.c16.as_str(), Ok("string"));
        assert_eq!(data.b16, *b"bytes\0\0\0");
        Ok(())
    };
    assert!(c.callback_add_mut(&mut |_ : &Channel, m : &Message| { r = check(m); 0 }, Some(MsgMask::Data as u32)).is_ok());

    assert!(c.open("").is_ok());

    assert_eq!(c.state(), State::Active);
    c.process()?;

    c.close();

    assert_eq!(c.state(), State::Closed);
    assert_eq!(r, Ok(()));

    Ok(())
}
