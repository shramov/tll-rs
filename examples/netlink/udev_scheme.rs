#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

pub use tll::scheme::*;

pub const SCHEME_STRING : &str = "yamls+gz://eJyNkd1qwkAQhe99irkbkATUWpDcpe0DlEIfYHVHXWpml+wmJci+uzNGLUiLvdpzdj7mtwQ2DVWA760P1KYBJwBbRwcbK1EAJRwvhD5YQBqCuphaxzvMd1BvDt1vVHmt80a925BWcbaC+UwEcddcqmG9Sc4zVnAcMzhOq+JMyBfW1kpoXgC+OFa5EPm6N7wjMU9iPqjxvZpnMZ+8HrHl2Xyx/9bUs5z/HNKM9W8D1Ffvg4qoXWhI0qB2hfl+A7FbxyEmah7uSrB/7dRSH0zaP+TCeEJH8Qed3u6aJyfOTZQG";


#[repr(C, packed(1))]
pub struct Property {
        pub name: tll::scheme::OffsetString,
        pub value: tll::scheme::OffsetString,
}
impl Binder for Property
{
    fn bind(data: &[u8]) -> Option<&Self>
    {
        if data.len() < std::mem::size_of::<Self>() { return None; }
        <tll::scheme::OffsetString as Binder>::bind(&data[0..])?; // name
        <tll::scheme::OffsetString as Binder>::bind(&data[8..])?; // value
        Some(unsafe { bind_unchecked::<Self>(data) })
    }
}

#[repr(i8)]
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum Action
{
        Unknown = 0,
        Add = 1,
        Bind = 2,
        Change = 3,
        Unbind = 4,
        Remove = 5,
}
impl Binder for Action {}

#[repr(C, packed(1))]
pub struct Device {
        pub action: Action,
        pub subsystem: tll::scheme::OffsetString,
        pub sysname: tll::scheme::OffsetString,
        pub devpath: tll::scheme::OffsetString,
        pub properties: tll::scheme::OffsetPtr<Property>,
}
impl MsgId for Device
{
        const MSGID : i32 = 10;
}
impl Binder for Device
{
    fn bind(data: &[u8]) -> Option<&Self>
    {
        if data.len() < std::mem::size_of::<Self>() { return None; }
        <Action as Binder>::bind(&data[0..])?; // action
        <tll::scheme::OffsetString as Binder>::bind(&data[1..])?; // subsystem
        <tll::scheme::OffsetString as Binder>::bind(&data[9..])?; // sysname
        <tll::scheme::OffsetString as Binder>::bind(&data[17..])?; // devpath
        <tll::scheme::OffsetPtr<Property> as Binder>::bind(&data[25..])?; // properties
        Some(unsafe { bind_unchecked::<Self>(data) })
    }
}

