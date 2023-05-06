#![allow(
    dead_code,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case
)]

pub use tll::scheme::*;

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    New = 0,
    Delete = 1,
}
impl Binder for Action {}

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RType {
    UNSPEC = 0,
    UNICAST = 1,
    LOCAL = 2,
    BROADCAST = 3,
    ANYCAST = 4,
    MULTICAST = 5,
    BLACKHOLE = 6,
    UNREACHABLE = 7,
    PROHIBIT = 8,
    THROW = 9,
    NAT = 10,
    XRESOLVE = 11,
    MAX = 12,
}
impl Binder for RType {}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct Link {
    pub pmap: u16,
    pub action: Action,
    pub type_: u16,
    pub type_raw: u16,
    pub index: i32,
    pub name: tll::scheme::ByteString<16>,
    pub up: u8,
}
impl MsgId for Link {
    const MSGID: i32 = 10;
}
impl Binder for Link {
    fn bind(data: &[u8]) -> Option<&Self> {
        if data.len() < std::mem::size_of::<Self>() {
            return None;
        }
        <Action as Binder>::bind(&data[0..])?; // action
        <i32 as Binder>::bind(&data[1..])?; // index
        <tll::scheme::ByteString<16> as Binder>::bind(&data[5..])?; // name
        <u8 as Binder>::bind(&data[21..])?; // up
        Some(unsafe { bind_unchecked::<Self>(data) })
    }
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct Route4 {
    pub action: Action,
    pub table: u32,
    pub type_: RType,
    pub oif: tll::scheme::ByteString<16>,
    pub dst_mask: u8,
    pub dst: u32,
    pub src_mask: u8,
    pub src: u32,
}
impl MsgId for Route4 {
    const MSGID: i32 = 20;
}
impl Binder for Route4 {
    fn bind(data: &[u8]) -> Option<&Self> {
        if data.len() < std::mem::size_of::<Self>() {
            return None;
        }
        <Action as Binder>::bind(&data[0..])?; // action
        <u32 as Binder>::bind(&data[1..])?; // table
        <RType as Binder>::bind(&data[5..])?; // type
        <tll::scheme::ByteString<16> as Binder>::bind(&data[6..])?; // oif
        <u8 as Binder>::bind(&data[22..])?; // dst_mask
        <u32 as Binder>::bind(&data[23..])?; // dst
        <u8 as Binder>::bind(&data[27..])?; // src_mask
        <u32 as Binder>::bind(&data[28..])?; // src
        Some(unsafe { bind_unchecked::<Self>(data) })
    }
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct Route6 {
    pub action: Action,
    pub table: u32,
    pub type_: RType,
    pub oif: tll::scheme::ByteString<16>,
    pub dst_mask: u8,
    pub dst: [u8; 16],
    pub src_mask: u8,
    pub src: [u8; 16],
}
impl MsgId for Route6 {
    const MSGID: i32 = 30;
}
impl Binder for Route6 {
    fn bind(data: &[u8]) -> Option<&Self> {
        if data.len() < std::mem::size_of::<Self>() {
            return None;
        }
        <Action as Binder>::bind(&data[0..])?; // action
        <u32 as Binder>::bind(&data[1..])?; // table
        <RType as Binder>::bind(&data[5..])?; // type
        <tll::scheme::ByteString<16> as Binder>::bind(&data[6..])?; // oif
        <u8 as Binder>::bind(&data[22..])?; // dst_mask
        <[u8; 16] as Binder>::bind(&data[23..])?; // dst
        <u8 as Binder>::bind(&data[39..])?; // src_mask
        <[u8; 16] as Binder>::bind(&data[40..])?; // src
        Some(unsafe { bind_unchecked::<Self>(data) })
    }
}
