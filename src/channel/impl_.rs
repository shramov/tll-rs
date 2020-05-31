use tll_sys::channel::*;
use tll_sys::channel::impl_::*;
use tll_sys::channel::callback::*;

use crate::channel::*;
use crate::props::*;

use crate::error::*;

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long, c_void};
//use std::option::Option;

#[ derive(Debug) ]
pub struct Internal {
    pub data : tll_channel_internal_t,
    c_name : CString,
    name : String,
}

impl Drop for Internal {
    fn drop(&mut self)
    {
        unsafe { tll_channel_internal_clear(&mut self.data) }
    }
}

impl Default for Internal {
    fn default() -> Self
    {
        let mut r = Internal { c_name: CString::default(), name: String::default(), data: unsafe { std::mem::zeroed::<tll_channel_internal_t>() } };
        unsafe {
            tll_channel_internal_init(&mut r.data);
            r.data.name = r.c_name.as_ptr();
        }
        r
    }
}

impl Internal {
    pub fn new() -> Self
    {
        Internal::default()
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn set_name(&mut self, n: &str) -> Result<()>
    {
        self.name = String::from(n);
        self.c_name = CString::new(n).ok().ok_or(Error::from("Name with internal NUL"))?;
        self.data.name = self.c_name.as_ptr();
        Ok(())
    }

    pub fn state(&self) -> State { State::from(self.data.state) }
    pub fn set_state(&mut self, state: State) -> State
    {
        let old = self.state();
        if old == state { return old; }
        println!("State change: {:?} -> {:?}", old, state);
        self.data.state = state.into();
        self.callback_simple(MsgType::State, self.data.state as i32);
        old
    }

    pub fn callback(&self, msg: &tll_msg_t)
    {
        channel_callback(&self.data, msg);
    }

    pub fn callback_data(&self, msg: &tll_msg_t)
    {
        channel_callback_data(&self.data, msg);
    }

    pub fn callback_simple(&self, t : MsgType, msgid: i32)
    {
        self.callback(Message::new().set_type(t).set_msgid(msgid))
    }

    pub fn init(&mut self, url: &Url) -> Result<()>
    {
        self.set_name(&url.get("name").unwrap_or("noname"))?;
        println!("New name: '{}' ({:?})", self.name(), self.c_name);
        Ok(())
    }

    /*
    impl<T> CImpl<T>
        where T : ChannelImpl
    pub fn open(&mut self, channel: &mut T, url: &Props) -> Result<()>
    {
    }
    */
}

pub struct CImpl<T : ChannelImpl> {
    data : tll_channel_impl_t,
    name : CString,
    phantom: std::marker::PhantomData<T>,
}

#[macro_export]
macro_rules! declare_channel_impl {
    ($var:ident, $klass:ident, $name:expr) => {
#[allow(dead_code, non_camel_case_types)]
#[doc(hidden)]
fn $var() -> &'static CImpl::<$klass>
{
    static mut POINTER: *const CImpl::<$klass> = std::ptr::null();
    static ONCE: std::sync::Once = std::sync::Once::new();

    unsafe {
        ONCE.call_once(|| {
            POINTER = std::mem::transmute(Box::new(CImpl::<$klass>::new($name)));
        });
        &*POINTER
    }
}

    }
}

impl<T> CImpl<T>
    where T : ChannelImpl
{
    pub fn new(name: &str) -> Self
    {
        let mut i = CImpl { data: unsafe { std::mem::zeroed() }, name: CString::new(name).unwrap(), phantom: std::marker::PhantomData };
        i.data.name = i.name.as_ptr();
        i.data.init = Some(Self::c_init);
        i.data.free = Some(Self::c_free);
        i.data.open = Some(Self::c_open);
        i.data.close = Some(Self::c_close);
        i.data.post = Some(Self::c_post);
        i.data.process = Some(Self::c_process);
        i
    }

    pub fn as_ptr(&self) -> * const tll_channel_impl_t { &self.data }

    pub fn name(&self) -> &CString { &self.name }

    fn init(c : &mut tll_channel_t, s: &[u8], parent: Option<Channel>, ctx: &Context) -> Result<()>
    {
        let surl = std::str::from_utf8(s)?;
        let url = Url::new(surl)?;
        c.data = Box::into_raw(Box::new(<T>::new())) as * mut c_void;
        println!("Call init on boxed object {:?}", c.data);
        //let mut channel = unsafe { std::ptr::NonNull::new_unchecked((*c).data as * mut T) };
        let channel = unsafe { &mut *((*c).data as * mut T) };
        let internal = channel.internal();
        internal.data.self_ = c;
        c.internal = &mut internal.data;
        println!("Call init on boxed object {:?}", c.data);
        internal.init(&url)?;
        let r = channel.init(&url, parent, ctx);
        println!("Init result: {:?}", r);
        if r.is_err() { Self::dealloc(c) };
        Ok(())
    }

    fn open(channel : &mut T, s: &[u8]) -> Result<()>
    {
        let surl = std::str::from_utf8(s).map_err(|_| format!("Invalid utf8 string {:?}", s))?;
        let url = Props::new(surl).map_err(|e| format!("Invalid props {:?}: {:?}'", surl, e))?;
        channel.open(&url)
    }

    extern "C" fn c_init(c : * mut tll_channel_t, s : * const c_char, len : usize, parent : * mut tll_channel_t, ctx : * mut tll_channel_context_t) -> c_int
    {
        if c.is_null() || s.is_null() || ctx.is_null() { return EINVAL as c_int }
        //if &self.data != unsafe { (*c).impl_ } { return EINVAL }
        match Self::init( unsafe { &mut *c },
                unsafe { std::slice::from_raw_parts(s as * const u8, len) },
                if parent.is_null() { None } else { Some(Channel::from_ptr(parent)) },
                    &Context::from(ctx)) {
            Ok(()) => 0,
            Err(r) => r.code.unwrap_or(EINVAL),
        }
    }

    fn dealloc(c : * mut tll_channel_t)
    {
        if c.is_null() { return () }
        unsafe {
            let data = (*c).data as * mut T;
            Box::<T>::from_raw(data);
            (*c).data = std::ptr::null_mut();
        }
    }

    extern "C" fn c_free(c : * mut tll_channel_t)
    {
        if c.is_null() || unsafe { (*c).data.is_null() } { return () }
        let channel = unsafe { &mut *((*c).data as * mut T) };
        channel.free();
        Self::dealloc(c)
    }

    extern "C" fn c_open(c : * mut tll_channel_t, s : * const c_char, len : usize) -> c_int
    {
        if c.is_null() || unsafe { (*c).data.is_null() } { return EINVAL }
        let channel = unsafe { &mut *((*c).data as * mut T) };
        match Self::open(channel, unsafe { std::slice::from_raw_parts(s as * const u8, len) }) {
            Err(e) => {
                println!("Open failed: {:?}", e);
                EINVAL
            },
            Ok(_) => 0,
        }
    }

    extern "C" fn c_close(c : * mut tll_channel_t) -> c_int
    {
        if c.is_null() || unsafe { (*c).data.is_null() } { return EINVAL }
        let channel = unsafe { &mut *((*c).data as * mut T) };
        channel.close();
        0
    }

    extern "C" fn c_post(c : * mut tll_channel_t, m : * const tll_msg_t, _ : c_int ) -> c_int
    {
        if c.is_null() || unsafe { (*c).data.is_null() } { return EINVAL }
        if m.is_null() { return EINVAL }
        let channel = unsafe { &mut *((*c).data as * mut T) };
        match channel.post(unsafe { &*(m as * const Message) }) {
            Ok(_) => 0,
            Err(_) => EINVAL,
        }
    }

    extern "C" fn c_process(c : * mut tll_channel_t, _ : c_long, _ : c_int) -> c_int
    {
        if c.is_null() || unsafe { (*c).data.is_null() } { return EINVAL }
        let channel = unsafe { &mut *((*c).data as * mut T) };
        match channel.process() {
            Ok(_) => 0,
            Err(_) => EINVAL,
        }
    }
}

pub trait ChannelImpl {
    fn new() -> Self;
    fn internal(&mut self) -> &mut Internal;
    fn init(&mut self, url: &Url, parent: Option<Channel>, context: &Context) -> Result<()>;
    fn open(&mut self, url: &Props) -> Result<()>;
    fn close(&mut self) {}
    fn free(&mut self) {}

    fn post(&mut self, _: &Message) -> Result<i32> { Ok(0) }
    fn process(&mut self) -> Result<i32> { Ok(EAGAIN) }
}
