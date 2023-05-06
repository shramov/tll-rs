use std::option::Option;

pub const EAGAIN : i32 = 11;
pub const EINVAL : i32 = 22;

#[ derive(Debug, Default, Clone, Eq, PartialEq) ]
pub struct Error
{
    pub code: Option<i32>,
    pub msg: String,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<&mut Error> for Error {
    fn from(e: &mut Error) -> Self { e.clone() }
}

impl From<String> for Error {
    fn from(e: String) -> Self { Error::from(&e as &str) }
}

impl From<std::str::Utf8Error> for Error {
    fn from(_: std::str::Utf8Error) -> Self { Error::from("Invalid utf-8 string") }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self { Error { code: None, msg: String::from(s) } }
}

impl From<i32> for Error {
    fn from(e: i32) -> Self { Error { code: Some(e), msg: format!("Error code {}", e) } }
}

pub fn error_check(r: i32) -> Result<()>
{
    match r {
        0 => Ok(()),
        e => Err(Error::from(e))
    }
}
