#![allow(
    dead_code,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case
)]

pub use tll::scheme::*;

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct relative {
    pub ts: tll::scheme::Duration<i64, tll::scheme::Nano>,
}
impl MsgId for relative {
    const MSGID: i32 = 1;
}
impl Binder for relative {
    fn bind(data: &[u8]) -> Option<&Self> {
        if data.len() < std::mem::size_of::<Self>() {
            return None;
        }
        <tll::scheme::Duration<i64, tll::scheme::Nano> as Binder>::bind(&data[0..])?; // ts
        Some(unsafe { bind_unchecked::<Self>(data) })
    }
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct absolute {
    pub ts: tll::scheme::TimePoint<i64, tll::scheme::Nano>,
}
impl MsgId for absolute {
    const MSGID: i32 = 2;
}
impl Binder for absolute {
    fn bind(data: &[u8]) -> Option<&Self> {
        if data.len() < std::mem::size_of::<Self>() {
            return None;
        }
        <tll::scheme::TimePoint<i64, tll::scheme::Nano> as Binder>::bind(&data[0..])?; // ts
        Some(unsafe { bind_unchecked::<Self>(data) })
    }
}
