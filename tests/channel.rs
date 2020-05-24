use tll::channel::*;
use tll::error::*;

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

fn callback(c: &Channel, m: &Message) -> i32
{
    println!("Callback: {} {:?} {:?}", c.name(), m.type_(), m.msgid());
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

        let c = r.as_mut()?.get_mut();
        assert_eq!(c.name(), "null");
        assert_eq!(c.state(), State::Closed);

        assert!(c.callback_add(&callback, None).is_ok());

        assert!(c.open("").is_ok());

        assert_eq!(c.state(), State::Active);

        //assert!(ctx.channel("null://;name=null").is_err()); // Check for duplicate name
    }

    assert!(ctx.channel("null://;name=null").is_ok());
    Ok(())
}
