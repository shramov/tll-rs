use tll_sys::channel::*;

use crate::config::Config;
use crate::channel::impl_::{ChannelImpl, CImpl};
use crate::error::*;

pub use crate::channel::message::*;
use std::ops::Deref;

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
//use std::option::Option;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Closed = TLL_STATE_CLOSED as isize,
    Opening = TLL_STATE_OPENING as isize,
    Active = TLL_STATE_ACTIVE as isize,
    Closing = TLL_STATE_CLOSING as isize,
    Error = TLL_STATE_ERROR as isize,
    Destroy = TLL_STATE_DESTROY as isize,
}

impl From<tll_state_t> for State {
    fn from(s: tll_state_t) -> Self
    {
        match s {
            TLL_STATE_CLOSED => State::Closed,
            TLL_STATE_OPENING => State::Opening,
            TLL_STATE_ACTIVE => State::Active,
            TLL_STATE_CLOSING => State::Closing,
            TLL_STATE_ERROR => State::Error,
            TLL_STATE_DESTROY => State::Destroy,
            _ => panic!("Invalid state {:?}", s),
        }
    }
}

impl Into<tll_state_t> for State {
    fn into(self) -> tll_state_t
    {
        self as tll_state_t
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MsgMask {
    All = TLL_MESSAGE_MASK_ALL as u32,
    Data = TLL_MESSAGE_MASK_DATA as u32,
    Constrol = TLL_MESSAGE_MASK_CONTROL as u32,
    State = TLL_MESSAGE_MASK_STATE as u32,
    Channel = TLL_MESSAGE_MASK_CHANNEL as u32,
}

#[ derive(Debug, PartialEq, Eq) ]
pub struct Context {
    ptr: *mut tll_channel_context_t
}

//pub type Message = tll_msg_t;
/*
#[ derive(Debug, PartialEq, Eq) ]
pub struct Message { ptr: * const tll_msg_t }

impl From<* const tll_msg_t> for Message {
    fn from(ptr: *const tll_msg_t) -> Self
    {
        assert!(!ptr.is_null());
        Message { ptr: ptr }
    }
}

impl Message {
    pub fn new_msg() -> tll_msg_t { unsafe { std::mem::zeroed() } }
    pub fn msgid(&self) -> i32 { unsafe { (*self.ptr).msgid } }
    pub fn type_(&self) -> MsgType { MsgType::from(unsafe { (*self.ptr).type_ } as tll_msg_type_t) }

    pub fn as_ref(&self) -> &tll_msg_t { unsafe { &*self.ptr } }
    pub fn data(&self) -> &[u8]
    {
        let size = unsafe { (*self.ptr).size };
        if size == 0 { return b""; }
        unsafe { std::slice::from_raw_parts((*self.ptr).data as * const u8, size) }
    }
}
*/

impl From<* mut tll_channel_context_t> for Context
{
    fn from(ptr: *mut tll_channel_context_t) -> Self
    {
        assert!(!ptr.is_null());
        unsafe { tll_channel_context_ref(ptr) };
        Context { ptr: ptr }
    }
}

impl Context {
    pub fn new() -> Self
    {
        let mut cfg = Config::new();
        let ptr = unsafe { tll_channel_context_new(cfg.as_mut_ptr()) };
        assert!(!ptr.is_null());
        Context { ptr: ptr }
    }

    fn consume(ptr: *mut tll_channel_context_t) -> Self
    {
        assert!(!ptr.is_null());
        Context { ptr: ptr }
    }

    pub fn as_ptr(&self) -> * mut tll_channel_context_t { self.ptr }

    pub fn config(&self) -> Config
    {
        Config::consume(unsafe { tll_channel_context_config(self.ptr) })
    }

    pub fn config_defaults(&self) -> Config
    {
        Config::consume(unsafe { tll_channel_context_config_defaults(self.ptr) })
    }

    pub fn channel(&self, url: &str) -> Result<OwnedChannel>
    {
        let ptr = unsafe { tll_channel_new(url.as_ptr() as *const c_char, url.len(), std::ptr::null_mut::<tll_channel_t>(), self.ptr) };
        if ptr.is_null() { Err(Error::from("Invalid argument")) } else { Ok(OwnedChannel(Channel {ptr: ptr})) }
    }

    pub fn register<T>(&self, impl_ : &'static CImpl::<T>) -> Result<()>
    where
        T : ChannelImpl
    {
        println!("Impl {:?} {:?}", impl_.name(), impl_.as_ptr());
        error_check(unsafe { tll_channel_impl_register(self.ptr, impl_.as_ptr(), impl_.name().as_ptr()) })
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { tll_channel_context_free(self.ptr); }
    }
}

#[ derive(Debug, PartialEq, Eq, PartialOrd, Ord) ]
pub struct Channel {
    ptr: *mut tll_channel_t
}

#[ derive(Debug, PartialEq, Eq) ]
pub struct OwnedChannel(Channel);

impl OwnedChannel {
    pub fn get(&self) -> &Channel { &self.0 }
    pub fn get_mut(&mut self) -> &mut Channel { &mut self.0 }
}

impl Drop for OwnedChannel {
    fn drop(&mut self) {
        unsafe { tll_channel_free(self.0.ptr); }
    }
}

impl std::ops::Deref for OwnedChannel {
    type Target = Channel;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl std::ops::DerefMut for OwnedChannel {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

extern "C" fn callback_wrap<F>(c : * const tll_channel_t, msg : * const tll_msg_t, user : * mut c_void ) -> c_int
    where F : FnMut(&Channel, &Message) -> i32
{
    if c.is_null() || msg.is_null() || user.is_null() { return 0 }
    let channel = Channel::from_ptr(c as * mut tll_channel_t);
    //let message = Message::from(msg);
    println!("Function: {:?}", user);
    let f = unsafe { &mut * (user as * mut F) };
    f(&channel, unsafe { &*(msg as * const Message) })
}

impl Channel {
    pub fn from_ptr(ptr: *mut tll_channel_t) -> Channel
    {
        assert!(!ptr.is_null());
        Channel { ptr: ptr }
    }

    pub fn state(&self) -> State
    {
        State::from( unsafe { tll_channel_state(self.ptr) } )
    }

    pub fn name<'a>(&'a self) -> &'a str
    {
        let n = unsafe { tll_channel_name(self.ptr) };
        if n.is_null() { "" } else {
            unsafe { CStr::from_ptr(n) }.to_str().unwrap()
        }
    }

    pub fn context(&self) -> Context
    {
        Context::consume(unsafe { tll_channel_context(self.ptr) })
    }

    pub fn config(&self) -> Config
    {
        Config::consume(unsafe { tll_channel_config(self.ptr) })
    }


    pub fn open(&mut self, props: &str) -> Result<()>
    {
        error_check(unsafe { tll_channel_open(self.ptr, props.as_ptr() as *const c_char, props.len()) })
    }

    pub fn close(&mut self) -> ()
    {
        unsafe { tll_channel_close(self.ptr) };
        ()
    }

    pub fn callback_add<F>(&mut self, f: &F, mask: Option<u32>) -> Result<()>
        where F : FnMut(&Channel, &Message) -> i32
    {
        let fptr = f as * const F as * mut F;
        println!("Callback add {:?}", fptr);
        error_check(unsafe { tll_channel_callback_add(self.ptr, Some(callback_wrap::<F>), fptr as * mut c_void, mask.unwrap_or(MsgMask::All as u32)) })
    }

    pub fn process(&mut self) -> Result<()>
    {
        error_check(unsafe { tll_channel_process(self.ptr, 0, 0) })
    }

    pub fn post(&mut self, msg : &Message) -> Result<()>
    {
        error_check(unsafe { tll_channel_post(self.ptr, msg.deref(), 0) })
    }
}
