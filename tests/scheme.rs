use tll::channel::*;
use tll::config::*;
use tll::error::*;

mod scheme_scheme;
use scheme_scheme::*;

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

    let url = Config::load_data("yamls", "
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
scheme: |
    yamls://
    - name: msg
      id: 10
      fields:
        - {name: i8, type: int8}
        - {name: u8, type: uint8}
        - {name: i16, type: int16}
        - {name: u16, type: uint16}
        - {name: i32, type: int32}
        - {name: u32, type: uint32}
        - {name: i64, type: int64}
        - {name: f64, type: double}
        - {name: d128, type: decimal128}
").ok_or("Failed to load config")?;

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
