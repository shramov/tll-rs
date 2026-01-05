use std::convert::TryFrom;
use tll::channel::*;
use tll::config::*;
use tll::{Error, Result};

mod scheme_scheme;
use crate::scheme_scheme::*;
//use crate::scheme_scheme::SCHEME_STRING;

use ::chrono::{TimeZone, Utc};

#[allow(dead_code)]
fn check(m: &Message) -> Result<()> {
    if m.get_type() != MsgType::Data {
        return Ok(());
    }
    println!("Callback: {:?} {:?}", m.get_type(), m.msgid);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let ctx = Context::new();

    let url = Config::load_data(
        "yamls",
        &format!(
            "
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
        u64: 1000000000
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
",
            SCHEME_STRING
        ),
    )?;

    let mut c = ctx.channel_url(&url)?;
    let mut r = Err(Error::from("No message received"));
    let check = |m: &Message| -> Result<()> {
        assert_eq!(m.msgid(), msg::<&[u8]>::MSGID);
        let data = msg::bind(m.data()).ok_or("Failed to bind")?;
        assert_eq!({ data.get_i8() }, -1);
        assert_eq!({ data.get_u8() }, 1);
        assert_eq!({ data.get_i16() }, -1000);
        assert_eq!({ data.get_u16() }, 1000);
        assert_eq!({ data.get_i32() }, -1000000);
        assert_eq!({ data.get_u32() }, 1000000);
        assert_eq!({ data.get_i64() }, -1000000000);
        assert_eq!({ data.get_u64() }, 1000000000);
        assert_eq!({ data.get_f64() }, 1.234);
        assert_eq!(format!("{}", { data.get_d128() }), "1234567890.E-5");
        assert_eq!(data.get_c16().as_str(), Ok("string"));
        assert_eq!(data.get_b8(), *b"bytes\0\0\0");
        assert_eq!(data.get_arr4().iter().collect::<Vec<_>>(), [1, 2, 3]);
        assert_eq!(data.get_ptr().unwrap().iter().collect::<Vec<_>>(), [10, 20, 30, 40]);
        assert_eq!(data.get_sub().get_s8(), 10);
        assert_eq!(
            std::time::Duration::try_from(data.get_duration_us()),
            Ok(std::time::Duration::from_micros(1234))
        );
        assert_eq!(
            std::time::Duration::try_from(data.get_duration_ns()),
            Ok(std::time::Duration::from_nanos(5432))
        );
        assert_eq!(
            data.get_timepoint_days().as_datetime(),
            Ok(Utc.datetime_from_str("2023-05-06 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap())
        );
        assert_eq!(
            data.get_timepoint_ns().as_datetime(),
            Ok(Utc.datetime_from_str("2023-05-06 12:34:56.000000789", "%Y-%m-%d %H:%M:%S.%f").unwrap())
        );
        Ok(())
    };
    assert!(c
        .callback_add_mut(
            &mut |_: &Channel, m: &Message| {
                r = check(m);
                0
            },
            Some(MsgMask::Data as u32)
        )
        .is_ok());

    assert!(c.open(None).is_ok());

    assert!(c.scheme().is_some());
    let scheme = c.scheme().unwrap();

    let mut imsg = scheme.messages();
    assert_eq!(imsg.next().map(|m| m.name()), Some("sub"));
    assert_eq!(imsg.next().map(|m| m.name()), Some("msg"));
    assert_eq!(imsg.next().map(|m| m.name()), None);

    let names: Vec<&str> = scheme.messages().map(|x| x.name()).collect();
    assert_eq!(names, ["sub", "msg"]);

    let mut omsg = scheme.messages().next();
    assert!(omsg.is_some());
    let mut msg = omsg.unwrap();

    {
        assert_eq!(msg.name(), "sub");
        assert_eq!(msg.size(), 1);
        assert_eq!(msg.msgid(), 0);
        let names = msg.fields().map(|x| x.name()).collect::<Vec<&str>>();
        assert_eq!(names, ["s8"]);
    }

    omsg = msg.next();
    assert!(omsg.is_some());
    msg = omsg.unwrap();
    assert_eq!(msg.name(), "msg");
    assert_eq!(msg.size(), 128);
    assert_eq!(msg.msgid(), 10);
    {
        use tll::scheme::scheme::{PointerVersion, SubType, TimeResolution, Type, TypeRaw};

        let names =
            msg.fields()
                .map(|x| (x.name(), x.type_raw(), x.size(), x.offset()))
                .collect::<Vec<(&str, TypeRaw, usize, usize)>>();
        assert_eq!(
            names,
            [
                ("i8", TypeRaw::Int8, 1, 0),
                ("u8", TypeRaw::UInt8, 1, 1),
                ("i16", TypeRaw::Int16, 2, 2),
                ("u16", TypeRaw::UInt16, 2, 4),
                ("i32", TypeRaw::Int32, 4, 6),
                ("u32", TypeRaw::UInt32, 4, 10),
                ("i64", TypeRaw::Int64, 8, 14),
                ("u64", TypeRaw::UInt64, 8, 22),
                ("f64", TypeRaw::Double, 8, 30),
                ("d128", TypeRaw::Decimal128, 16, 38),
                ("c16", TypeRaw::Bytes, 16, 54),
                ("b8", TypeRaw::Bytes, 8, 70),
                ("arr4", TypeRaw::Array, 1 + 4 * 4, 78),
                ("ptr", TypeRaw::Pointer, 8, 95),
                ("sub", TypeRaw::Message, 1, 103),
                ("duration_us", TypeRaw::Int32, 4, 104),
                ("duration_ns", TypeRaw::UInt64, 8, 108),
                ("timepoint_days", TypeRaw::Int32, 4, 116),
                ("timepoint_ns", TypeRaw::UInt64, 8, 120),
            ]
        );

        let types = msg
            .fields()
            .filter_map(|x| match x.sub_type() {
                SubType::None => None,
                t => Some((x.name(), t)),
            })
            .collect::<Vec<(&str, SubType)>>();
        assert_eq!(
            types,
            [
                ("c16", SubType::ByteString),
                ("duration_us", SubType::Duration(TimeResolution::Us)),
                ("duration_ns", SubType::Duration(TimeResolution::Ns)),
                ("timepoint_days", SubType::TimePoint(TimeResolution::Day)),
                ("timepoint_ns", SubType::TimePoint(TimeResolution::Ns)),
            ]
        );
        let sub = msg.fields().find(|x| x.name() == "sub").unwrap();
        assert_eq!(sub.type_ptr(), None);
        assert_eq!(sub.type_msg().map(|x| x.name()), Some("sub"));
        assert_eq!(sub.type_msg(), scheme.messages().find(|x| x.name() == "sub"));
        assert_eq!(sub.get_type(), Type::Message(sub.type_msg().unwrap()));

        assert_eq!(
            msg.fields().find(|x| x.name() == "b8").as_ref().map(|x| x.get_type()),
            Some(Type::Bytes(8))
        );

        let mut f = msg.fields().find(|x| x.name() == "arr4").unwrap();
        match f.get_type() {
            Type::Array {
                capacity,
                counter,
                data,
            } => {
                assert_eq!(capacity, 4);
                assert_eq!(counter.name(), "arr4_count");
                assert_eq!(counter.get_type(), Type::Int8);
                assert_eq!(data.name(), "arr4");
                assert_eq!(data.get_type(), Type::Int32);
                assert_eq!(data.offset(), counter.size());
            }
            t => panic!("Invalid array type: {:?}", t),
        }

        f = msg.fields().find(|x| x.name() == "ptr").unwrap();
        match f.get_type() {
            Type::Pointer { version, data } => {
                assert_eq!(version, PointerVersion::Default);
                assert_eq!(data.name(), "ptr");
                assert_eq!(data.get_type(), Type::Int64);
            }
            t => panic!("Invalid array type: {:?}", t),
        }
    }

    assert!(msg.next().is_none());

    assert_eq!(c.state(), State::Active);
    c.process()?;

    c.close();

    assert_eq!(c.state(), State::Closed);
    assert_eq!(r, Ok(()));

    Ok(())
}
