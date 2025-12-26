use tll_sys::channel::*;
use std::os::raw::c_void;
use std::os::raw::c_short;
use std::ops::Deref;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MsgType {
    Data,
    Control,
    State,
    Channel,
    Unknown(i16),
}

impl From<c_short> for MsgType {
    #[inline(always)]
    fn from(x: i16) -> Self
    {
        match x as tll_msg_type_t {
            TLL_MESSAGE_DATA => MsgType::Data,
            TLL_MESSAGE_CONTROL => MsgType::Control,
            TLL_MESSAGE_STATE => MsgType::State,
            TLL_MESSAGE_CHANNEL => MsgType::Channel,
            r => MsgType::Unknown(r as i16)
        }
    }
}

impl From<MsgType> for c_short {
    #[inline(always)]
    fn from(value: MsgType) -> c_short
    {
        match value {
            MsgType::Data => TLL_MESSAGE_DATA as c_short,
            MsgType::Control => TLL_MESSAGE_CONTROL as c_short,
            MsgType::State => TLL_MESSAGE_STATE as c_short,
            MsgType::Channel => TLL_MESSAGE_CHANNEL as c_short,
            MsgType::Unknown(r) => r as c_short
        }
    }
}

#[ repr(C) ]
#[ derive(Debug) ]
pub struct Message(tll_msg_t);

impl Default for Message {
    #[inline(always)]
    fn default() -> Self { Message (unsafe { std::mem::zeroed() }) }
}

impl Deref for Message {
    type Target = tll_msg_t;
    #[inline(always)]
    fn deref(&self) -> &tll_msg_t { &self.0 }
}

impl Message {
    #[inline(always)]
    pub fn new() -> Self { Message::default() }

    #[inline(always)]
    pub fn data(&self) -> &[u8]
    {
        if self.data.is_null() { return b"" }
        unsafe { std::slice::from_raw_parts(self.0.data as * const u8, self.size) }
    }

    #[inline(always)]
    pub fn set_data(&mut self, data : &[u8]) -> &mut Self
    {
        self.0.size = data.len();
        self.0.data = data.as_ptr() as * const c_void;
        self
    }

    #[inline(always)]
    pub fn set_data_from_ref<T : Sized>(&mut self, data : &T) -> &mut Self
    {
        self.0.size = std::mem::size_of::<T>();
        self.0.data = data as * const T as * const c_void;
        self
    }

    #[inline(always)]
    pub fn get_type(&self) -> MsgType { MsgType::from(self.0.type_) }
    #[inline(always)]
    pub fn set_type(&mut self, t : MsgType) -> &mut Self
    {
        self.0.type_ = t.into();
        self
    }

    #[inline(always)]
    pub fn msgid(&self) -> i32 { self.0.msgid }
    #[inline(always)]
    pub fn set_msgid(&mut self, id : i32) -> &mut Self
    {
        self.0.msgid = id;
        self
    }

    #[inline(always)]
    pub fn seq(&self) -> i64 { self.0.seq }
    #[inline(always)]
    pub fn set_seq(&mut self, seq : i64) -> &mut Self
    {
        self.0.seq = seq;
        self
    }

    #[inline(always)]
    pub fn addr(&self) -> u64 { unsafe { self.0.addr.u64_ } }
    #[inline(always)]
    pub fn set_addr(&mut self, addr : u64) -> &mut Self
    {
        self.0.addr.u64_ = addr;
        self
    }
}

#[test]
fn test_set_data() {
    let mut msg = Message::default();
    let i = 0xbeefu32;
    msg.set_data_from_ref(&i);
    assert_eq!(msg.size, 4);
    assert_eq!(msg.data(), b"\xef\xbe\0\0");
}
