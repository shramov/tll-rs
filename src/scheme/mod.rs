pub mod chrono;
pub mod native;
pub mod scheme;
pub mod serde;

pub use self::chrono::*;
pub use self::scheme::Scheme;

pub trait MsgId {
    const MSGID: i32;
}
