#![allow(
    dead_code,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case
)]

pub use tll::scheme::*;

pub const SCHEME_STRING : &str = "yamls+gz://eJylktuKgzAQhu/7FHMXKCusrRTxVZZFYpOWAY2Sw4WUvvtmBN0c7LLL3mXMl2/yZyxA8UE2wIzr2AHghrIXpvErgAIe62bN3sDOE61R2Zo9D8V6cDB3OoiigfL9pQFzQwy4AHC7BJaXyOHLTBIibp/B8ynS+DLThIjbZ/BSRRpfpsgtRMToul5mjChPQXAhrzjwnr6l4DWM1s1WLvU4WRyVaeDBaIvRrKxGdWfPVNDV8fm8Bde6Sp7mo/rMsMnqb+q4H53+po2hIsvtNKert86k04hSaWnG3lFJ2RZ4S7o68qybXZl4kMtEXurVL/UWBzmNXtcKPv/l/h4PO5CnXUQ/9fhPhrjDF/4MKXc=";

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
        <f64 as Binder>::bind(&data[22..])?; // f64
        <tll::decimal128::Decimal128 as Binder>::bind(&data[30..])?; // d128
        <tll::scheme::ByteString<16> as Binder>::bind(&data[46..])?; // c16
        <[u8; 8] as Binder>::bind(&data[62..])?; // b8
        <tll::scheme::Array<i32, i8, 4> as Binder>::bind(&data[70..])?; // arr4
        <tll::scheme::OffsetPtr<i64> as Binder>::bind(&data[87..])?; // ptr
        <sub as Binder>::bind(&data[95..])?; // sub
        <tll::scheme::Duration<i32, tll::scheme::Micro> as Binder>::bind(&data[96..])?; // duration_us
        <tll::scheme::Duration<u64, tll::scheme::Nano> as Binder>::bind(&data[100..])?; // duration_ns
        <tll::scheme::TimePoint<i32, tll::scheme::RatioDay> as Binder>::bind(&data[108..])?; // timepoint_days
        <tll::scheme::TimePoint<u64, tll::scheme::Nano> as Binder>::bind(&data[112..])?; // timepoint_ns
        Some(unsafe { bind_unchecked::<Self>(data) })
    }
}
