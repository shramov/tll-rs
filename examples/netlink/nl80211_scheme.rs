#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

pub use tll::scheme::*;

pub const SCHEME_STRING : &str = "yamls+gz://eJyFjrEOgjAQhnef4rZbSgKYGNPNxMXFzQeo9DBNoBBaooT03b0ChoiDU//cff3uT8CqmiTgxXrqSlUQ7gCMlpClHMj2tZMcAPBUeNNYlDD6oeUvxvqjmAge4Zkq8sTbXABe6ckp5XRrtZrGWQisKQ1VehEmMC631WwWMIs/l8IGM1bTa6V6LpAdfqj4rNB98MSQgKaNTherxhU3Quc7Yx8YtgbnjP427PN/hjdmFmH1";

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    New = 0,
    Update = 1,
    Delete = 2,
}

impl Binder for Action {}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct Interface {
    pub action: Action,
    pub index: u16,
    pub name: tll::scheme::ByteString<16>,
    pub ssid: tll::scheme::ByteString<32>,
}
impl MsgId for Interface {
    const MSGID: i32 = 10;
}
impl Binder for Interface {
    fn bind(data: &[u8]) -> Option<&Self> {
        if data.len() < std::mem::size_of::<Self>() {
            return None;
        }
        <Action as Binder>::bind(&data[0..])?; // action
        <u16 as Binder>::bind(&data[1..])?; // index
        <tll::scheme::ByteString<16> as Binder>::bind(&data[3..])?; // name
        <tll::scheme::ByteString<32> as Binder>::bind(&data[19..])?; // ssid
        Some(unsafe { bind_unchecked::<Self>(data) })
    }
}
