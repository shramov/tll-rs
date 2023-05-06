use tll::channel::*;
use tll::config::*;
use tll::error::{Error, Result};
use std::convert::TryFrom;

mod scheme_scheme;
use crate::scheme_scheme::*;
//use crate::scheme_scheme::SCHEME_STRING;

use ::chrono::{Utc, TimeZone};

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
        c16: string
        b8: bytes
        arr4: [1, 2, 3]
        ptr: [10, 20, 30, 40]
        sub.s8: 10
        duration_us: 1234us
        duration_ns: 5432ns
        timepoint_days: 2023-05-06
        timepoint_ns: 2023-05-06T12:34:56.0000000789
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
        assert_eq!(data.b8, *b"bytes\0\0\0");
        assert_eq!(data.arr4.data(), [1, 2, 3]);
        assert_eq!(unsafe { data.ptr.data() }, [10, 20, 30, 40]);
        assert_eq!(data.sub.s8, 10);
        assert_eq!(std::time::Duration::try_from(data.duration_us), Ok(std::time::Duration::from_micros(1234)));
        assert_eq!(std::time::Duration::try_from(data.duration_ns), Ok(std::time::Duration::from_nanos(5432)));
        assert_eq!(data.timepoint_days.as_datetime(), Ok(Utc.datetime_from_str("2023-05-06 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()));
        assert_eq!(data.timepoint_ns.as_datetime(), Ok(Utc.datetime_from_str("2023-05-06 12:34:56.000000789", "%Y-%m-%d %H:%M:%S.%f").unwrap()));
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
