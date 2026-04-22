use tll::channel::*;
use tll::Result;

/*
#[ derive(Debug, PartialEq, Eq, PartialOrd, Ord) ]
struct Error(i32);

impl From<i32> for Error {
    fn from(v: i32) -> Self { Error(v) }
}

impl From<&mut i32> for Error {
    fn from(v: &mut i32) -> Self { Error(*v) }
}
*/

#[allow(dead_code)]
fn callback(c: &Channel, m: &Message) -> i32 {
    println!("Callback: {} {:?} {:?}", c.name(), m.type_, m.msgid);
    0
}

#[test]
fn test() -> Result<()> {
    let ctx = Context::new();
    assert!(ctx.channel("invalid-proto://").is_err());
    assert!(ctx.channel("null://;invalid;").is_err());

    {
        let mut r = ctx.channel("null://;name=null");
        assert!(r.is_ok());

        let c = r.as_mut()?;
        let cfg = c.config();

        assert_eq!(c.name(), "null");
        assert_eq!(c.state(), State::Closed);
        assert_eq!(cfg.get("state"), Some(String::from("Closed")));

        let mut last = (MsgType::Data, 0i32);
        let mut cb = |_: &Channel, m: &Message| {
            last = (m.get_type(), m.msgid());
            0
        };
        let cb = c.callback_add_mut(&mut cb, MsgMask::All);
        assert!(cb.is_ok());

        assert!(c.open(None).is_ok());

        assert_eq!(c.state(), State::Active);
        assert_eq!(cfg.get("state"), Some(String::from("Active")));
        assert_eq!(last, (MsgType::State, c.state() as i32));

        c.close();

        assert_eq!(c.state(), State::Closed);
        assert_eq!(cfg.get("state"), Some(String::from("Closed")));
        assert_eq!(last, (MsgType::State, c.state() as i32));

        //assert!(ctx.channel("null://;name=null").is_err()); // Check for duplicate name
    }

    assert!(ctx.channel("null://;name=null").is_ok());
    Ok(())
}

#[test]
fn test_context_scheme() -> Result<()> {
    let ctx = Context::new();

    let mut c0 = ctx.channel("null://;name=c0;scheme=yamls://[{name: Data, id: 10}]")?;

    assert!(ctx.scheme_load("channel://c0").is_err());
    c0.open(None)?;
    assert_eq!(c0.state(), State::Active);
    assert!(c0.scheme().is_some());
    assert!(ctx.scheme_load("channel://c0").is_ok());

    let mut c1 = ctx.channel("null://;name=c1;scheme=channel://c0")?;
    c1.open(None)?;
    assert_eq!(c1.state(), State::Active);
    assert!(c1.scheme().is_some());

    Ok(())
}
