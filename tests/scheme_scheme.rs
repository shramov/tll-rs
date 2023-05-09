#![allow(
    dead_code,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case
)]

pub use tll::scheme::*;

pub const SCHEME_STRING : &str = "yamls+gz://eJylkt2KgzAQRu/7FLkLlBXWVor4KssisUnLgEbJz4WUvvtmBN3EyZZd9i5jTs7ky1gwLQbVMG59xw+M3UD10jZhxVjBHutmzd+Ymydcg3Y1fx6K9eBg73gQZMPK9x8NQA0p4CPAZwkoL4kjlEQSIz7PwPmUaEJJNDHi8wxcqkQTSqKJEZ9nbjEjR9/1ijCyPEWPI9UVBtHjtz14jeN3s1NLPU4ORm0b9uC4xXGezoC+8+de0NXpedpCGFPtnu+j+iTY5Mw3dcxHxz9uY7Agub0RePXW2/3EklRG2bH3WGK2Bd6Srg6adbNrS6b0Qq9/qXcwqGkMulaK+S/3D3jcAT3tInrV4z8Z0g5fMvYzTg==";

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct sub {
    pub s8: i8,
}
impl Binder for sub {
    fn bind(data: &[u8]) -> Option<&Self> {
        if data.len() < std::mem::size_of::<Self>() {
            return None;
        }
        <i8 as Binder>::bind(&data[0..])?; // s8
        Some(unsafe { bind_unchecked::<Self>(data) })
    }
}

#[repr(C, packed(1))]
pub struct msg {
    pub i8: i8,
    pub u8: u8,
    pub i16: i16,
    pub u16: u16,
    pub i32: i32,
    pub u32: u32,
    pub i64: i64,
    pub u64: u64,
    pub f64: f64,
    pub d128: tll::decimal128::Decimal128,
    pub c16: tll::scheme::ByteString<16>,
    pub b8: [u8; 8],
    pub arr4: tll::scheme::Array<i32, i8, 4>,
    pub ptr: tll::scheme::OffsetPtr<i64>,
    pub sub: sub,
    pub duration_us: tll::scheme::Duration<i32, tll::scheme::Micro>,
    pub duration_ns: tll::scheme::Duration<u64, tll::scheme::Nano>,
    pub timepoint_days: tll::scheme::TimePoint<i32, tll::scheme::RatioDay>,
    pub timepoint_ns: tll::scheme::TimePoint<u64, tll::scheme::Nano>,
}
impl MsgId for msg {
    const MSGID: i32 = 10;
}
impl Binder for msg {
    fn bind(data: &[u8]) -> Option<&Self> {
        if data.len() < std::mem::size_of::<Self>() {
            return None;
        }
        <i8 as Binder>::bind(&data[0..])?; // i8
        <u8 as Binder>::bind(&data[1..])?; // u8
        <i16 as Binder>::bind(&data[2..])?; // i16
        <u16 as Binder>::bind(&data[4..])?; // u16
        <i32 as Binder>::bind(&data[6..])?; // i32
        <u32 as Binder>::bind(&data[10..])?; // u32
        <i64 as Binder>::bind(&data[14..])?; // i64
        <u64 as Binder>::bind(&data[22..])?; // u64
        <f64 as Binder>::bind(&data[30..])?; // f64
        <tll::decimal128::Decimal128 as Binder>::bind(&data[38..])?; // d128
        <tll::scheme::ByteString<16> as Binder>::bind(&data[54..])?; // c16
        <[u8; 8] as Binder>::bind(&data[70..])?; // b8
        <tll::scheme::Array<i32, i8, 4> as Binder>::bind(&data[78..])?; // arr4
        <tll::scheme::OffsetPtr<i64> as Binder>::bind(&data[95..])?; // ptr
        <sub as Binder>::bind(&data[103..])?; // sub
        <tll::scheme::Duration<i32, tll::scheme::Micro> as Binder>::bind(&data[104..])?; // duration_us
        <tll::scheme::Duration<u64, tll::scheme::Nano> as Binder>::bind(&data[108..])?; // duration_ns
        <tll::scheme::TimePoint<i32, tll::scheme::RatioDay> as Binder>::bind(&data[116..])?; // timepoint_days
        <tll::scheme::TimePoint<u64, tll::scheme::Nano> as Binder>::bind(&data[120..])?; // timepoint_ns
        Some(unsafe { bind_unchecked::<Self>(data) })
    }
}
