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

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteString16 {
    data: [u8; 16],
}

impl tll::scheme::ByteString for ByteString16 {
    fn get_data(&self) -> &[u8] {
        &self.data
    }
}
#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct Link {
    pub action: Action,
    pub index: i32,
    pub name: ByteString16,
    pub up: u8,
}
impl MsgId for Link {
    const MSGID: i32 = 10;
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct Route4 {
    pub action: Action,
    pub table: u32,
    pub type_: RType,
    pub oif: ByteString16,
    pub dst_mask: u8,
    pub dst: u32,
    pub src_mask: u8,
    pub src: u32,
}
impl MsgId for Route4 {
    const MSGID: i32 = 20;
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct Route6 {
    pub action: Action,
    pub table: u32,
    pub type_: RType,
    pub oif: ByteString16,
    pub dst_mask: u8,
    pub dst: [u8; 16],
    pub src_mask: u8,
    pub src: [u8; 16],
}
impl MsgId for Route6 {
    const MSGID: i32 = 30;
}
