#![allow(
    dead_code,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case
)]

pub use tll::scheme::*;

pub const SCHEME_STRING : &str = "yamls+gz://eJx1kk0KgzAQRveeYnaBUqFaKZKrlC6MiTKgUTRZiHj3JgXb/HWXL/PmhUmSg2xGQYGsmpEMoEMx8JWaFUAO+1msyRXUNts1SlWTI8vPxnHtbSNyCsXtrwFjgw9oB9BJAouH5zAxkriITjN4Lz2NiZHGRXSawUflaUwMkc5F+KTZICKGF6UzOBctjs1g90KwdUdjmxKfPM0KJ7lS2IktEftWakHZkyMUsNrvj49olqUKruZZvSJsVsuPuqRHt7/py9hwZG9RuKsj";

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
    pub f64: f64,
    pub d128: tll::decimal128::Decimal128,
    pub c16: tll::scheme::ByteString<16>,
    pub b8: [u8; 8],
    pub arr4: tll::scheme::Array<i32, i8, 4>,
    pub ptr: tll::scheme::OffsetPtr<i64>,
    pub sub: sub,
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
        <f64 as Binder>::bind(&data[22..])?; // f64
        <tll::decimal128::Decimal128 as Binder>::bind(&data[30..])?; // d128
        <tll::scheme::ByteString<16> as Binder>::bind(&data[46..])?; // c16
        <[u8; 8] as Binder>::bind(&data[62..])?; // b8
        <tll::scheme::Array<i32, i8, 4> as Binder>::bind(&data[70..])?; // arr4
        <tll::scheme::OffsetPtr<i64> as Binder>::bind(&data[87..])?; // ptr
        <sub as Binder>::bind(&data[95..])?; // sub
        Some(unsafe { bind_unchecked::<Self>(data) })
    }
}
