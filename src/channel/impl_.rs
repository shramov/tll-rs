use tll_sys::channel::*;
use tll_sys::channel_callback::*;
use tll_sys::config::tll_config_t;

// Reexport for using in macro
pub use tll_sys::channel::{tll_channel_module_t, tll_channel_context_t};

use crate::channel::*;
use crate::config::*;

use crate::error::*;
use crate::logger::*;

use std::ffi::CString;
use std::os::raw::{c_int, c_long, c_void};
//use std::option::Option;

#[ derive(Debug, Eq, PartialEq) ]
pub enum ProcessPolicy {
    Normal,
    Never,
}

#[ derive(Debug, Eq, PartialEq) ]
pub enum OpenPolicy {
    Normal,
    Manual,
}

#[ derive(Debug, Eq, PartialEq) ]
pub enum ChildPolicy {
    Never,
    Single,
    Many,
}

#[repr(C)]
#[ derive(Debug) ]
struct Stat {
    rx: crate::stat::Field,
    rxb: crate::stat::Field,
    tx: crate::stat::Field,
    txb: crate::stat::Field,
}

impl Default for Stat {
    fn default() -> Self
    {
        Self {
            rx: crate::stat::Field::new("rx", crate::stat::Type::Sum),
            rxb: crate::stat::Field::new_unit("tx", crate::stat::Type::Sum, crate::stat::Unit::Bytes),
            tx: crate::stat::Field::new("rx", crate::stat::Type::Sum),
            txb: crate::stat::Field::new_unit("tx", crate::stat::Type::Sum, crate::stat::Unit::Bytes),
        }
    }
}

#[ derive(Debug) ]
pub struct Base {
    pub data : tll_channel_internal_t,
    c_name : CString,
    name : String,
    pub logger : Logger,
    stat : Option<crate::stat::Base<Stat>>
}

impl Drop for Base {
    fn drop(&mut self)
    {
        unsafe { tll_channel_internal_clear(&mut self.data) }
    }
}

impl Default for Base {
    fn default() -> Self
    {
        let mut r = Base { c_name: CString::default(), name: String::default(), logger: Logger::new("rust.channel"), data: unsafe { std::mem::zeroed::<tll_channel_internal_t>() }, stat: None };
        unsafe {
            tll_channel_internal_init(&mut r.data);
            r.data.name = r.c_name.as_ptr();
        }
        r
    }
}

impl Base {
    pub fn new() -> Self
    {
        Base::default()
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

    pub fn init_base(&mut self, url: &Config) -> Result<()>
    {
        self.set_name(&url.get("name").unwrap_or("noname".to_string()))?;
        self.logger = Logger::new(&format!("rust.channel.{}", self.name));
        println!("New name: '{}' ({:?})", self.name(), self.c_name);
        if url.get_typed("stat", false)? {
            let mut stat = crate::stat::Base::<Stat>::new(self.name());
            self.data.stat = stat.as_ptr();
            self.stat = Some(stat);
        }
        Ok(())
    }

    pub fn caps(&self) -> Caps
    {
        Caps::from_bits_truncate(self.data.caps)
    }

    pub fn set_caps(&mut self, caps: Caps)
    {
        self.data.caps = caps.bits();
    }

    pub fn update_dcaps(&mut self, caps: DCaps, mask: DCaps)
    {
        let old = self.data.dcaps;
        println!("Update dcaps: {} -> {:?}", old, caps);
        if old & mask.bits() == caps.bits() { return; }
        self.data.dcaps ^= (old & mask.bits()) ^ caps.bits();
        let mut msg = Message::new();
        msg.set_type(MsgType::Channel).set_msgid(TLL_MESSAGE_CHANNEL_UPDATE as i32).set_data(&old.to_ne_bytes());
        self.callback(&msg);
    }

    /*
    impl<T> CImpl<T>
        where T : ChannelImpl
    pub fn open(&mut self, channel: &mut T, url: &Props) -> Result<()>
    {
    }
    */
}

pub trait ChannelImpl : Extension {
    fn process_policy() -> ProcessPolicy { <Self::Inner as ChannelImpl>::process_policy() }
    fn open_policy() -> OpenPolicy { <Self::Inner as ChannelImpl>::open_policy() }
    fn child_policy() -> ChildPolicy { <Self::Inner as ChannelImpl>::child_policy() }

    fn init(&mut self, url: &Config, master: Option<Channel>, context: &Context) -> Result<()> { self.inner_mut().init(url, master, context) }
    fn open(&mut self, url: &Config) -> Result<()> { self.inner_mut().open(url) }
    fn close(&mut self, force : bool) { self.inner_mut().close(force) }
    fn free(&mut self) { self.inner_mut().free() }

    fn post(&mut self, msg: &Message) -> Result<()> { self.inner_mut().post(msg) }
    fn process(&mut self) -> Result<i32> { self.inner_mut().process() }

    fn logger(&self) -> &Logger { self.base().logger() }
    fn state(&self) -> State { self.base().state() }
    fn set_state(&mut self, state: State) -> State { self.base_mut().set_state(state) }
    fn update_dcaps(&mut self, caps: DCaps, mask: DCaps) { self.base_mut().update_dcaps(caps, mask) }
}

pub trait Extension : Default {
    type Inner : ChannelImpl;

    fn base(&self) -> &Base { self.inner().base() }
    fn base_mut(&mut self) -> &mut Base { self.inner_mut().base_mut() }

    fn inner(&self) -> &Self::Inner;
    fn inner_mut(&mut self) -> &mut Self::Inner;
}

impl Extension for Base {
    type Inner = Base;
    fn base(&self) -> &Base { self }
    fn base_mut(&mut self) -> &mut Base { self }

    fn inner(&self) -> &Self::Inner { self }
    fn inner_mut(&mut self) -> &mut Self::Inner { self }
}

impl ChannelImpl for Base {
    fn process_policy() -> ProcessPolicy { ProcessPolicy::Normal }
    fn open_policy() -> OpenPolicy { OpenPolicy::Normal }
    fn child_policy() -> ChildPolicy { ChildPolicy::Never }

    fn init(&mut self, _url: &Config, _master: Option<Channel>, _context: &Context) -> Result<()> { Ok(()) }
    fn open(&mut self, _url: &Config) -> Result<()> { Ok(()) }
    fn close(&mut self, _force : bool) {}
    fn free(&mut self) {}

    fn post(&mut self, _: &Message) -> Result<()> { Ok(()) }
    fn process(&mut self) -> Result<i32> { Ok(EAGAIN) }

    fn logger(&self) -> &Logger { &self.logger }
    fn state(&self) -> State { self.state() }
    fn set_state(&mut self, state: State) -> State { self.set_state(state) }
    fn update_dcaps(&mut self, caps: DCaps, mask: DCaps) { self.update_dcaps(caps, mask) }
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

    fn init(c : &mut tll_channel_t, url: &Config, master: Option<Channel>, ctx: &Context) -> Result<()>
    {
        c.data = Box::into_raw(Box::new(<T>::default())) as * mut c_void;
        let log = Logger::new(&format!("tll.channel.{}", url.get("name").unwrap_or(String::from("noname"))));
        //println!("Call init on boxed object {:?}", c.data);
        //let mut channel = unsafe { std::ptr::NonNull::new_unchecked((*c).data as * mut T) };
        let channel = unsafe { &mut *((*c).data as * mut T) };
        let internal = channel.base_mut();
        internal.data.self_ = c;
        c.internal = &mut internal.data;
        println!("Call init on boxed object {:?}", c.data);
        if let Err(e) = internal.init_base(url) {
            log.error(&format!("Base init failed: {:?}", e));
            return Err(e);
        }
        internal.set_caps(match <T>::child_policy() {
            ChildPolicy::Never => Caps::empty(),
            ChildPolicy::Single => Caps::Parent | Caps::Proxy,
            ChildPolicy::Many => Caps::Parent,
        });
        let r = channel.init(&url, master, ctx);
        println!("Init result: {:?}", r);
        if r.is_err() { Self::dealloc(c); return r; };
        Ok(())
    }

    fn open(channel : &mut T, cfg: &Config) -> Result<()>
    {
        channel.set_state(State::Opening);
        match <T>::process_policy() {
            ProcessPolicy::Normal => channel.update_dcaps(DCaps::Process, DCaps::Process),
            ProcessPolicy::Never => ()
        }

        let r = channel.open(cfg);
        if r.is_ok() {
            if <T>::open_policy() == OpenPolicy::Normal { channel.set_state(State::Active); }
        }
        r
    }

    extern "C" fn c_init(c : * mut tll_channel_t, url : * const tll_config_t, master : * mut tll_channel_t, ctx : * mut tll_channel_context_t) -> c_int
    {
        if c.is_null() || url.is_null() || ctx.is_null() { return EINVAL as c_int }
        //if &self.data != unsafe { (*c).impl_ } { return EINVAL }
        match Self::init( unsafe { &mut *c },
                &Config::from(url as * mut tll_config_t),
                if master.is_null() { None } else { Some(Channel::from_ptr(master)) },
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

    extern "C" fn c_open(c : * mut tll_channel_t, url : * const tll_config_t) -> c_int
    {
        if c.is_null() || unsafe { (*c).data.is_null() } { return EINVAL }
        let channel = unsafe { &mut *((*c).data as * mut T) };
        let cfg = if url.is_null() { Config::new() } else { Config::from(url as * mut tll_config_t) };
        match Self::open(channel, &cfg) {
            Err(e) => {
                println!("Open failed: {:?}", e);
                EINVAL
            },
            Ok(_) => 0,
        }
    }

    extern "C" fn c_close(c : * mut tll_channel_t, force : c_int) -> c_int
    {
        if c.is_null() || unsafe { (*c).data.is_null() } { return EINVAL }
        let channel = unsafe { &mut *((*c).data as * mut T) };
        channel.close(force != 0);
        channel.base_mut().update_dcaps(DCaps::empty(), DCaps::Process | DCaps::POLLMASK);
        channel.set_state(State::Closed);
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

#[macro_export]
macro_rules! declare_channel_module {
    ( $( $impl0:ident ), * ) => {
unsafe extern "C" fn _channel_module_init(_m: *mut tll_channel_module_t, ctx: *mut tll_channel_context_t) -> std::os::raw::c_int
{
    $(
    if let Err(e) = Context::from(ctx).register($impl0()) { return e.code.unwrap_or(EINVAL); };
    )*
    0
}

unsafe extern "C" fn _channel_module_free(_m: *mut tll_channel_module_t, ctx: *mut tll_channel_context_t) -> std::os::raw::c_int
{
    /*
    $(
    if let Err(_) = Context::from(ctx).unregister($impl0()) { assert!(false, "Failed to unload"); };
    )*
    */
    0
}

#[no_mangle]
unsafe extern "C" fn channel_module() -> *const tll_channel_module_t
{
    static mut MODULE : tll_channel_module_t =
        tll_channel_module_t {
                version: 1,
                flags: 0,
                init: Some(_channel_module_init),
                free: Some(_channel_module_free),
                impl_: std::ptr::null_mut(),
        };
    &MODULE
}
    }
}
