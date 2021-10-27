#![allow(
    dead_code,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case
)]

pub use tll::scheme::*;

pub const SCHEME_STRING : &str = "yamls+gz://eJxtkMsKgzAQRff9itnNpkK1IpJfKV2oiTKgUUyyEPHfO0oXebi7N3NywiQD3UxKAE5mwAcASQH5i0NPapRGcALIYP9TVOMT7LZcWdsajwhwHuBuCcqrwME1kfiIu2foXQQaronGR9w9Q1UZaLjGSO8jcnbtqBJG5oW3uFQdTc14nsVg56/WblZdfV4szdoI2PEcIc+MXUkPeMSCtg7vp08061pGX/Mpv4z9AISah90=";

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
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
        Some(unsafe { bind_unchecked::<Self>(data) })
    }
}
