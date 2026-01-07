#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

pub use tll::bind::*;

pub const SCHEME_STRING : &str = "yamls+gz://eJyFkD0PgjAQhnd+xW1dIKGYGMJm4uLi5myQHtoESkNLlJD+d698BMXBqW96T5+3uQhUXmMGjAUAjbayUSaDgRVaR35idF4go7mq0jjh/GqKB9bIXBAtL0/KYlt6jBRSZMBjCqi62mQUANih8F6yDLbX9EQqm4Yj4auOWKH1HUkI7IxPSjGlixb5eM2dI00psRKzMIJh7s4ncwiTeGlyG0wqga+V6ugDfP9D+WOFbr1FgsLPrfiRX4axrVR35rYGY6T4NuySf4Y3hqFz+g==";

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    New = 0,
    Update = 1,
    Delete = 2,
}
impl BinderCopy for Action {}

#[derive(Debug)]
pub struct Interface<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> Binder<Buf> for Interface<Buf> {
    fn bind_view(data: MemOffset<Buf>) -> Result<Self, BindError> {
        if data.mem_size() < 51 {
            return Err(BindError::new_size(51));
        }
        Ok(Self { data })
    }

    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        Self { data }
    }
}

impl<Buf: MemRead + Copy> Interface<Buf> {
    pub fn get_action(&self) -> Action {
        self.data.mem_get_primitive::<Action>(0)
    }
    pub fn get_index(&self) -> u16 {
        self.data.mem_get_primitive::<u16>(1)
    }
    pub fn get_name(&self) -> Result<&'_ str, StringBindError> {
        tll::bind::byte_str(&self.data, 3, 16)
    }
    pub fn get_ssid(&self) -> Result<&'_ str, StringBindError> {
        tll::bind::byte_str(&self.data, 19, 32)
    }
}
impl<Buf: MemRead> MsgId for Interface<Buf> {
    const MSGID: i32 = 10;
}
