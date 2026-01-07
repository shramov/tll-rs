#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

pub use tll::bind::*;

pub const SCHEME_STRING : &str = "yamls+gz://eJy1jTEKwzAMRfec4m9e4qGlZPBliosVEDi2iZVCCbl7pdD2Bp30P3rS8yhxoQC3Uo7CT3IDwCngonNmyqkHTYDH/iGluxHyapa5yHTTWptwLT1g10e95s2qU6AY7Iy2lrY1npvjGPzXHB/nwc98/YdZeKF7q3pl7jeV6ktb";

#[derive(Debug)]
pub struct relative<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> relative<Buf> {
    pub fn bind(data: Buf) -> std::result::Result<Self, BindError> {
        if data.mem_size() < 8 {
            return Err(BindError::new_size(8));
        }
        Ok(Self {
            data: MemOffset::new(data),
        })
    }

    pub fn bind_unchecked(data: Buf) -> Self {
        Self {
            data: MemOffset::new(data),
        }
    }

    pub fn get_ts(&self) -> tll::scheme::Duration<i64, tll::scheme::Nano> {
        self.data.mem_get_primitive::<tll::scheme::Duration<i64, tll::scheme::Nano>>(0)
    }
}
impl<Buf: MemRead> MsgId for relative<Buf> {
    const MSGID: i32 = 1;
}
#[derive(Debug)]
pub struct absolute<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> absolute<Buf> {
    pub fn bind(data: Buf) -> std::result::Result<Self, BindError> {
        if data.mem_size() < 8 {
            return Err(BindError::new_size(8));
        }
        Ok(Self {
            data: MemOffset::new(data),
        })
    }

    pub fn bind_unchecked(data: Buf) -> Self {
        Self {
            data: MemOffset::new(data),
        }
    }

    pub fn get_ts(&self) -> tll::scheme::TimePoint<i64, tll::scheme::Nano> {
        self.data.mem_get_primitive::<tll::scheme::TimePoint<i64, tll::scheme::Nano>>(0)
    }
}
impl<Buf: MemRead> MsgId for absolute<Buf> {
    const MSGID: i32 = 2;
}
