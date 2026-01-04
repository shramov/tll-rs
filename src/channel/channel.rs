use tll_sys::channel::*;

use crate::channel::base::{CImpl, ChannelImpl};
use crate::config::Config;
use crate::error::*;
use crate::scheme::Scheme;

pub use crate::channel::caps::*;
pub use crate::channel::message::*;

use std::convert::TryFrom;
use std::ops::Deref;

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_short, c_void};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Closed = TLL_STATE_CLOSED as isize,
    Opening = TLL_STATE_OPENING as isize,
    Active = TLL_STATE_ACTIVE as isize,
    Closing = TLL_STATE_CLOSING as isize,
    Error = TLL_STATE_ERROR as isize,
    Destroy = TLL_STATE_DESTROY as isize,
}

impl TryFrom<i32> for State {
    type Error = crate::error::Error;
    fn try_from(s: i32) -> std::result::Result<Self, crate::error::Error> {
        match s as u32 {
            TLL_STATE_CLOSED => Ok(State::Closed),
            TLL_STATE_OPENING => Ok(State::Opening),
            TLL_STATE_ACTIVE => Ok(State::Active),
            TLL_STATE_CLOSING => Ok(State::Closing),
            TLL_STATE_ERROR => Ok(State::Error),
            TLL_STATE_DESTROY => Ok(State::Destroy),
            _ => Err(Error::from(format!("Invalid state {:?}", s))),
        }
    }
}

impl From<tll_state_t> for State {
    fn from(s: tll_state_t) -> Self {
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

impl From<State> for tll_state_t {
    fn from(v: State) -> Self {
        v as tll_state_t
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MsgMask {
    All = TLL_MESSAGE_MASK_ALL as u32,
    Data = TLL_MESSAGE_MASK_DATA as u32,
    Control = TLL_MESSAGE_MASK_CONTROL as u32,
    State = TLL_MESSAGE_MASK_STATE as u32,
    Channel = TLL_MESSAGE_MASK_CHANNEL as u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Context {
    ptr: *mut tll_channel_context_t,
}

impl From<*mut tll_channel_context_t> for Context {
    fn from(ptr: *mut tll_channel_context_t) -> Self {
        assert!(!ptr.is_null());
        unsafe { tll_channel_context_ref(ptr) };
        Context { ptr: ptr }
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        unsafe { tll_channel_context_ref(self.ptr) };
        Context { ptr: self.ptr }
    }
}

impl Context {
    pub fn new() -> Self {
        Self::new_cfg(Config::new())
    }

    pub fn new_cfg(mut cfg: Config) -> Self {
        let ptr = unsafe { tll_channel_context_new(cfg.as_mut_ptr()) };
        assert!(!ptr.is_null());
        Context { ptr: ptr }
    }

    fn consume(ptr: *mut tll_channel_context_t) -> Self {
        assert!(!ptr.is_null());
        Context { ptr: ptr }
    }

    pub fn as_ptr(&self) -> *mut tll_channel_context_t {
        self.ptr
    }

    pub fn config(&self) -> Config {
        Config::consume(unsafe { tll_channel_context_config(self.ptr) })
    }

    pub fn config_defaults(&self) -> Config {
        Config::consume(unsafe { tll_channel_context_config_defaults(self.ptr) })
    }

    pub fn channel(&self, url: &str) -> Result<OwnedChannel> {
        let ptr = unsafe {
            tll_channel_new(
                self.ptr,
                url.as_ptr() as *const c_char,
                url.len(),
                std::ptr::null_mut::<tll_channel_t>(),
                std::ptr::null::<tll_channel_impl_t>(),
            )
        };
        if ptr.is_null() {
            Err(Error::from("Failed to create channel"))
        } else {
            Ok(OwnedChannel(Channel { ptr: ptr }))
        }
    }

    pub fn channel_url(&self, url: &Config) -> Result<OwnedChannel> {
        let ptr = unsafe {
            tll_channel_new_url(
                self.ptr,
                url.as_ptr(),
                std::ptr::null_mut::<tll_channel_t>(),
                std::ptr::null::<tll_channel_impl_t>(),
            )
        };
        if ptr.is_null() {
            Err(Error::from("Failed to create channel"))
        } else {
            Ok(OwnedChannel(Channel { ptr: ptr }))
        }
    }

    pub fn register<T>(&self, impl_: &'static CImpl<T>) -> Result<()>
    where
        T: ChannelImpl,
    {
        error_check_str(
            unsafe { tll_channel_impl_register(self.ptr, impl_.as_ptr(), impl_.name().as_ptr()) },
            "Failed to register impl",
        )
    }

    pub fn unregister<T>(&self, impl_: &'static CImpl<T>) -> Result<()>
    where
        T: ChannelImpl,
    {
        error_check_str(
            unsafe { tll_channel_impl_unregister(self.ptr, impl_.as_ptr(), impl_.name().as_ptr()) },
            "Failed to unregister impl",
        )
    }

    pub fn load(&self, module: &str) -> Result<()> {
        let cmodule = CString::new(module).map_err(|_| Error::from("Internal null byte in module string"))?;
        if unsafe { tll_channel_module_load(self.ptr, cmodule.as_ptr(), std::ptr::null()) } != 0 {
            Err(Error::from(format!("Failed to load module '{}'", module)))
        } else {
            Ok(())
        }
    }

    pub fn load_symbol(&self, module: &str, symbol: &str) -> Result<()> {
        let cmodule = CString::new(module).map_err(|_| Error::from("Internal null byte in module string"))?;
        let csymbol = CString::new(symbol).map_err(|_| Error::from("Internal null byte in symbol string"))?;
        if unsafe { tll_channel_module_load(self.ptr, cmodule.as_ptr(), csymbol.as_ptr()) } != 0 {
            Err(Error::from(format!(
                "Failed to load module '{}' symbol '{}'",
                module, symbol
            )))
        } else {
            Ok(())
        }
    }

    pub fn get(&self, name: &str) -> Option<Channel> {
        let ptr = unsafe { tll_channel_get(self.ptr, name.as_ptr() as *const c_char, name.len() as i32) };
        if ptr.is_null() {
            None
        } else {
            Some(Channel { ptr: ptr })
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            tll_channel_context_free(self.ptr);
        }
        self.ptr = std::ptr::null_mut();
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Channel {
    ptr: *mut tll_channel_t,
}

#[derive(Debug, PartialEq, Eq)]
pub struct OwnedChannel(Channel);

impl OwnedChannel {
    pub fn get(&self) -> &Channel {
        &self.0
    }
    pub fn get_mut(&mut self) -> &mut Channel {
        &mut self.0
    }

    pub unsafe fn new_null() -> Self {
        OwnedChannel(Channel {
            ptr: std::ptr::null_mut(),
        })
    }
}

impl Drop for OwnedChannel {
    fn drop(&mut self) {
        unsafe {
            tll_channel_free(self.0.ptr);
        }
        self.0.ptr = std::ptr::null_mut();
    }
}

impl std::ops::Deref for OwnedChannel {
    type Target = Channel;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedChannel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait CallbackMut<Tag = ()> {
    fn message_callback_mut(&mut self, c: &Channel, m: &Message) -> i32;
}

pub trait Callback<Tag = ()> {
    fn message_callback(&self, c: &Channel, m: &Message) -> i32;
}

impl<T: FnMut(&Channel, &Message) -> i32> CallbackMut<()> for T {
    fn message_callback_mut(&mut self, c: &Channel, m: &Message) -> i32 {
        self(c, m)
    }
}

impl<T: Fn(&Channel, &Message) -> i32> Callback<()> for T {
    fn message_callback(&self, c: &Channel, m: &Message) -> i32 {
        self(c, m)
    }
}

//impl<T : Callback> CallbackMut for T {
//    fn message_callback_mut(&mut self, c: &Channel, m: &Message) -> i32 { self.message_callback(c, m) }
//}

/*
struct Dumper {}
impl Callback<()> for Dumper {
    fn message_callback(&self, _c: &Channel, m: &Message) -> i32 {
        println!("Message {:?}", m);
        0
    }
}
*/

extern "C" fn callback_wrap_mut<F, T>(c: *const tll_channel_t, msg: *const tll_msg_t, user: *mut c_void) -> c_int
where
    F: CallbackMut<T>,
{
    if c.is_null() || msg.is_null() || user.is_null() {
        return 0;
    }
    let channel = Channel::from_ptr(c as *mut tll_channel_t);
    //let message = Message::from(msg);
    //println!("Function: {:?}", user);
    let f = unsafe { &mut *(user as *mut F) };
    f.message_callback_mut(&channel, unsafe { &*(msg as *const Message) })
}

extern "C" fn callback_wrap<F, T>(c: *const tll_channel_t, msg: *const tll_msg_t, user: *mut c_void) -> c_int
where
    F: Callback<T>,
{
    if c.is_null() || msg.is_null() || user.is_null() {
        return 0;
    }
    let channel = Channel::from_ptr(c as *mut tll_channel_t);
    //let message = Message::from(msg);
    //println!("Function: {:?}", user);
    let f = unsafe { &mut *(user as *mut F) };
    f.message_callback(&channel, unsafe { &*(msg as *const Message) })
}

impl Channel {
    pub fn from_ptr(ptr: *mut tll_channel_t) -> Channel {
        assert!(!ptr.is_null());
        Channel { ptr: ptr }
    }

    pub fn as_ptr(&mut self) -> *mut tll_channel_t {
        self.ptr
    }
    pub fn as_const_ptr(&self) -> *const tll_channel_t {
        self.ptr
    }

    pub fn state(&self) -> State {
        State::from(unsafe { tll_channel_state(self.ptr) })
    }

    pub fn caps(&self) -> Caps {
        Caps::from_bits_truncate(unsafe { tll_channel_caps(self.ptr) })
    }

    pub fn dcaps(&self) -> DCaps {
        DCaps::from_bits_truncate(unsafe { tll_channel_dcaps(self.ptr) })
    }

    pub fn name<'a>(&'a self) -> &'a str {
        let n = unsafe { tll_channel_name(self.ptr) };
        if n.is_null() {
            ""
        } else {
            unsafe { CStr::from_ptr(n) }.to_str().unwrap()
        }
    }

    pub fn context(&self) -> Context {
        Context::consume(unsafe { tll_channel_context(self.ptr) })
    }

    pub fn config(&self) -> Config {
        Config::consume(unsafe { tll_channel_config(self.ptr) })
    }

    pub fn scheme(&self) -> Option<Scheme> {
        self.scheme_type(MsgType::Data)
    }
    pub fn scheme_control(&self) -> Option<Scheme> {
        self.scheme_type(MsgType::Control)
    }
    pub fn scheme_type(&self, type_: MsgType) -> Option<Scheme> {
        let ptr = unsafe { tll_channel_scheme(self.ptr, c_short::from(type_) as c_int) };
        if ptr.is_null() {
            None
        } else {
            Some(Scheme::from(ptr))
        }
    }

    pub fn open(&mut self, props: &str) -> Result<()> {
        error_check_str(
            unsafe { tll_channel_open(self.ptr, props.as_ptr() as *const c_char, props.len()) },
            "Failed to open channel",
        )
    }

    pub fn open_cfg(&mut self, cfg: &Config) -> Result<()> {
        error_check_str(
            unsafe { tll_channel_open_cfg(self.ptr, cfg.as_ptr()) },
            "Failed to open channel",
        )
    }

    pub fn close(&mut self) -> () {
        self.close_force(false)
    }

    pub fn close_force(&mut self, force: bool) -> () {
        unsafe { tll_channel_close(self.ptr, force as i32) };
        ()
    }

    pub fn callback_add<F, T>(&mut self, f: &F, mask: Option<u32>) -> Result<()>
    where
        F: Callback<T>,
    {
        let fptr = f as *const F as *mut F;
        let r = unsafe {
            tll_channel_callback_add(
                self.ptr,
                Some(callback_wrap::<F, T>),
                fptr as *mut c_void,
                mask.unwrap_or(MsgMask::All as u32),
            )
        };
        error_check_str(r, "Failed to add callback")
    }

    pub fn callback_add_mut<F, T>(&mut self, f: &mut F, mask: Option<u32>) -> Result<()>
    where
        F: CallbackMut<T>,
    {
        let fptr = f as *mut F;
        let r = unsafe {
            tll_channel_callback_add(
                self.ptr,
                Some(callback_wrap_mut::<F, T>),
                fptr as *mut c_void,
                mask.unwrap_or(MsgMask::All as u32),
            )
        };
        error_check_str(r, "Failed to add callback")
    }

    pub fn callback_del<F, Tag>(&mut self, f: &F, mask: Option<u32>) -> Result<()>
    where
        F: Callback<Tag>,
    {
        let fptr = f as *const F as *mut F;
        let r = unsafe {
            tll_channel_callback_del(
                self.ptr,
                Some(callback_wrap::<F, Tag>),
                fptr as *mut c_void,
                mask.unwrap_or(MsgMask::All as u32),
            )
        };
        error_check_str(r, "Failed to del callback")
    }

    pub fn callback_del_mut<F, Tag>(&mut self, f: &F, mask: Option<u32>) -> Result<()>
    where
        F: CallbackMut<Tag>,
    {
        let fptr = f as *const F;
        let r = unsafe {
            tll_channel_callback_del(
                self.ptr,
                Some(callback_wrap_mut::<F, Tag>),
                fptr as *mut c_void,
                mask.unwrap_or(MsgMask::All as u32),
            )
        };
        error_check_str(r, "Failed to del callback")
    }

    pub fn process(&mut self) -> Result<i32> {
        match unsafe { tll_channel_process(self.ptr, 0, 0) } {
            0 => Ok(0),
            EAGAIN => Ok(EAGAIN),
            e => Err(Error::from(e)),
        }
    }

    pub fn post(&mut self, msg: &Message) -> Result<()> {
        error_check_str(unsafe { tll_channel_post(self.ptr, msg.deref(), 0) }, "Post failed")
    }

    pub fn post_flags(&mut self, msg: &Message, flags: PostFlags) -> Result<()> {
        error_check_str(
            unsafe { tll_channel_post(self.ptr, msg.deref(), flags.bits() as i32) },
            "Post failed",
        )
    }
}
