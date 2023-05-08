use std::option::Option;

pub const EAGAIN : i32 = 11;
pub const EINVAL : i32 = 22;

#[ derive(Debug, Default, Clone, Eq, PartialEq) ]
pub struct Error
{
    pub code: Option<i32>,
    pub msg: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.code {
            Some(code) => write!(f, "{}, code: {}", self.msg, code),
            None => write!(f, "{}", self.msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<&mut Error> for Error {
    fn from(e: &mut Error) -> Self { e.clone() }
}

impl From<String> for Error {
    fn from(e: String) -> Self { Error::from(&e as &str) }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self { Error { code: None, msg: String::from(s) } }
}

impl From<i32> for Error {
    fn from(e: i32) -> Self { Error { code: Some(e), msg: format!("Error code {}", e) } }
}

impl From<std::str::Utf8Error> for Error {
    fn from(_: std::str::Utf8Error) -> Self { Error::from("Invalid utf-8 string") }
}

impl From<std::io::Error> for Error
{
    fn from(e: std::io::Error) -> Self { Self::from(format!("IO error: {}", e)) }
}

pub fn error_check(r: i32) -> Result<()>
{
    match r {
        0 => Ok(()),
        e => Err(Error::from(e))
    }
}

pub fn error_check_str(r: i32, message: &str) -> Result<()>
{
    match r {
        0 => Ok(()),
        e => Err(Error { code: Some(e), msg: message.to_string() })
    }
}

pub fn error_check_fn<F>(r: i32, func: F) -> Result<()>
where
    F : FnOnce() -> String,
{
    match r {
        0 => Ok(()),
        e => Err(Error { code: Some(e), msg: func() })
    }
}
