/* automatically generated by rust-bindgen 0.59.1 */

#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]
use crate::config::tll_config_t;
use crate::scheme::tll_scheme_t;
use crate::stat::*;

pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub const TLL_STATE_CLOSED: tll_state_t = 0;
pub const TLL_STATE_OPENING: tll_state_t = 1;
pub const TLL_STATE_ACTIVE: tll_state_t = 2;
pub const TLL_STATE_CLOSING: tll_state_t = 3;
pub const TLL_STATE_ERROR: tll_state_t = 4;
pub const TLL_STATE_DESTROY: tll_state_t = 5;
pub type tll_state_t = ::std::os::raw::c_uint;
pub const TLL_MESSAGE_DATA: tll_msg_type_t = 0;
pub const TLL_MESSAGE_CONTROL: tll_msg_type_t = 1;
pub const TLL_MESSAGE_STATE: tll_msg_type_t = 2;
pub const TLL_MESSAGE_CHANNEL: tll_msg_type_t = 3;
pub type tll_msg_type_t = ::std::os::raw::c_uint;
pub const TLL_MESSAGE_CHANNEL_UPDATE: tll_msg_channel_t = 0;
pub const TLL_MESSAGE_CHANNEL_ADD: tll_msg_channel_t = 1;
pub const TLL_MESSAGE_CHANNEL_DELETE: tll_msg_channel_t = 2;
pub const TLL_MESSAGE_CHANNEL_UPDATE_FD: tll_msg_channel_t = 3;
pub type tll_msg_channel_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub union tll_addr_t {
    pub u64_: u64,
    pub i64_: i64,
    pub ptr: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_tll_addr_t() {
    assert_eq!(
        ::std::mem::size_of::<tll_addr_t>(),
        8usize,
        concat!("Size of: ", stringify!(tll_addr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tll_addr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tll_addr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_addr_t>())).u64_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_addr_t),
            "::",
            stringify!(u64_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_addr_t>())).i64_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_addr_t),
            "::",
            stringify!(i64_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_addr_t>())).ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_addr_t),
            "::",
            stringify!(ptr)
        )
    );
}
impl ::std::fmt::Debug for tll_addr_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "tll_addr_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tll_msg_t {
    pub type_: ::std::os::raw::c_short,
    pub msgid: ::std::os::raw::c_int,
    pub seq: ::std::os::raw::c_longlong,
    pub flags: ::std::os::raw::c_short,
    pub data: *const ::std::os::raw::c_void,
    pub size: usize,
    pub addr: tll_addr_t,
    pub time: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout_tll_msg_t() {
    assert_eq!(
        ::std::mem::size_of::<tll_msg_t>(),
        56usize,
        concat!("Size of: ", stringify!(tll_msg_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tll_msg_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tll_msg_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_msg_t>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_msg_t),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_msg_t>())).msgid as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_msg_t),
            "::",
            stringify!(msgid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_msg_t>())).seq as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_msg_t),
            "::",
            stringify!(seq)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_msg_t>())).flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_msg_t),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_msg_t>())).data as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_msg_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_msg_t>())).size as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_msg_t),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_msg_t>())).addr as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_msg_t),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_msg_t>())).time as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_msg_t),
            "::",
            stringify!(time)
        )
    );
}
impl ::std::fmt::Debug for tll_msg_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! ( f , "tll_msg_t {{ type: {:?}, msgid: {:?}, seq: {:?}, flags: {:?}, data: {:?}, size: {:?}, addr: {:?}, time: {:?} }}" , self . type_ , self . msgid , self . seq , self . flags , self . data , self . size , self . addr , self . time )
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tll_channel_context_t {
    _unused: [u8; 0],
}
pub const TLL_CAPS_INPUT: tll_channel_cap_t = 4;
pub const TLL_CAPS_OUTPUT: tll_channel_cap_t = 8;
pub const TLL_CAPS_INOUT: tll_channel_cap_t = 12;
pub const TLL_CAPS_EX_BIT: tll_channel_cap_t = 8388608;
pub const TLL_CAPS_CUSTOM: tll_channel_cap_t = 8388608;
pub const TLL_CAPS_PARENT: tll_channel_cap_t = 8388609;
pub const TLL_CAPS_PROXY: tll_channel_cap_t = 8388610;
pub const TLL_CAPS_LONG_CLOSE: tll_channel_cap_t = 8388612;
pub type tll_channel_cap_t = ::std::os::raw::c_uint;
pub const TLL_DCAPS_ZERO: tll_channel_dcap_t = 0;
pub const TLL_DCAPS_POLLIN: tll_channel_dcap_t = 1;
pub const TLL_DCAPS_POLLOUT: tll_channel_dcap_t = 2;
pub const TLL_DCAPS_POLLMASK: tll_channel_dcap_t = 3;
pub const TLL_DCAPS_PROCESS: tll_channel_dcap_t = 16;
pub const TLL_DCAPS_PENDING: tll_channel_dcap_t = 32;
pub const TLL_DCAPS_SUSPEND: tll_channel_dcap_t = 64;
pub const TLL_DCAPS_SUSPEND_PERMANENT: tll_channel_dcap_t = 128;
pub const TLL_DCAPS_PROCESS_MASK: tll_channel_dcap_t = 80;
pub type tll_channel_dcap_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tll_channel_list_t {
    pub channel: *mut tll_channel_t,
    pub next: *mut tll_channel_list_t,
}
#[test]
fn bindgen_test_layout_tll_channel_list_t() {
    assert_eq!(
        ::std::mem::size_of::<tll_channel_list_t>(),
        16usize,
        concat!("Size of: ", stringify!(tll_channel_list_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tll_channel_list_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tll_channel_list_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_list_t>())).channel as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_list_t),
            "::",
            stringify!(channel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_list_t>())).next as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_list_t),
            "::",
            stringify!(next)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tll_channel_t {
    pub impl_: *const tll_channel_impl_t,
    pub data: *mut ::std::os::raw::c_void,
    pub internal: *mut tll_channel_internal_t,
    pub context: *mut tll_channel_context_t,
    pub parent: *mut tll_channel_t,
}
#[test]
fn bindgen_test_layout_tll_channel_t() {
    assert_eq!(
        ::std::mem::size_of::<tll_channel_t>(),
        40usize,
        concat!("Size of: ", stringify!(tll_channel_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tll_channel_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tll_channel_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_t>())).impl_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_t),
            "::",
            stringify!(impl_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_t>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_t>())).internal as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_t),
            "::",
            stringify!(internal)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_t>())).context as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_t),
            "::",
            stringify!(context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_t>())).parent as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_t),
            "::",
            stringify!(parent)
        )
    );
}
pub const TLL_MESSAGE_MASK_ALL: tll_message_mask_t = 4294967295;
pub const TLL_MESSAGE_MASK_DATA: tll_message_mask_t = 1;
pub const TLL_MESSAGE_MASK_CONTROL: tll_message_mask_t = 2;
pub const TLL_MESSAGE_MASK_STATE: tll_message_mask_t = 4;
pub const TLL_MESSAGE_MASK_CHANNEL: tll_message_mask_t = 8;
pub type tll_message_mask_t = ::std::os::raw::c_uint;
pub type tll_channel_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        channel: *const tll_channel_t,
        msg: *const tll_msg_t,
        user: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn tll_channel_callback_add(
        arg1: *mut tll_channel_t,
        cb: tll_channel_callback_t,
        user: *mut ::std::os::raw::c_void,
        mask: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_callback_del(
        arg1: *mut tll_channel_t,
        cb: tll_channel_callback_t,
        user: *mut ::std::os::raw::c_void,
        mask: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_new(
        ctx: *mut tll_channel_context_t,
        str_: *const ::std::os::raw::c_char,
        len: usize,
        master: *mut tll_channel_t,
        impl_: *const tll_channel_impl_t,
    ) -> *mut tll_channel_t;
}
extern "C" {
    pub fn tll_channel_new_url(
        ctx: *mut tll_channel_context_t,
        url: *const tll_config_t,
        master: *mut tll_channel_t,
        impl_: *const tll_channel_impl_t,
    ) -> *mut tll_channel_t;
}
extern "C" {
    pub fn tll_channel_free(arg1: *mut tll_channel_t);
}
extern "C" {
    pub fn tll_channel_open(
        arg1: *mut tll_channel_t,
        str_: *const ::std::os::raw::c_char,
        len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_open_cfg(
        arg1: *mut tll_channel_t,
        cfg: *const tll_config_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_close(
        arg1: *mut tll_channel_t,
        force: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_process(
        c: *mut tll_channel_t,
        timeout: ::std::os::raw::c_long,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub const TLL_POST_MORE: tll_channel_post_flag_t = 1;
pub type tll_channel_post_flag_t = ::std::os::raw::c_uint;
extern "C" {
    pub fn tll_channel_post(
        c: *mut tll_channel_t,
        msg: *const tll_msg_t,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_suspend(c: *mut tll_channel_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_resume(c: *mut tll_channel_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_state(c: *const tll_channel_t) -> tll_state_t;
}
extern "C" {
    pub fn tll_channel_name(c: *const tll_channel_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn tll_channel_caps(c: *const tll_channel_t) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn tll_channel_dcaps(c: *const tll_channel_t) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn tll_channel_fd(c: *const tll_channel_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_context(c: *const tll_channel_t) -> *mut tll_channel_context_t;
}
extern "C" {
    pub fn tll_channel_config(c: *mut tll_channel_t) -> *mut tll_config_t;
}
extern "C" {
    pub fn tll_channel_children(c: *const tll_channel_t) -> *const tll_channel_list_t;
}
extern "C" {
    pub fn tll_channel_scheme(
        c: *const tll_channel_t,
        type_: ::std::os::raw::c_int,
    ) -> *const tll_scheme_t;
}
extern "C" {
    pub fn tll_channel_get(
        ctx: *const tll_channel_context_t,
        name: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> *mut tll_channel_t;
}
extern "C" {
    pub fn tll_channel_context_new(defaults: *mut tll_config_t) -> *mut tll_channel_context_t;
}
extern "C" {
    pub fn tll_channel_context_ref(arg1: *mut tll_channel_context_t) -> *mut tll_channel_context_t;
}
extern "C" {
    pub fn tll_channel_context_default() -> *mut tll_channel_context_t;
}
extern "C" {
    pub fn tll_channel_context_free(arg1: *mut tll_channel_context_t);
}
extern "C" {
    pub fn tll_channel_context_config(arg1: *mut tll_channel_context_t) -> *mut tll_config_t;
}
extern "C" {
    pub fn tll_channel_context_config_defaults(
        arg1: *mut tll_channel_context_t,
    ) -> *mut tll_config_t;
}
extern "C" {
    pub fn tll_channel_context_stat_list(arg1: *mut tll_channel_context_t) -> *mut tll_stat_list_t;
}
extern "C" {
    pub fn tll_channel_context_scheme_load(
        ctx: *mut tll_channel_context_t,
        url: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        cache: ::std::os::raw::c_int,
    ) -> *const tll_scheme_t;
}
extern "C" {
    pub fn tll_channel_impl_register(
        ctx: *mut tll_channel_context_t,
        impl_: *const tll_channel_impl_t,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_impl_unregister(
        ctx: *mut tll_channel_context_t,
        impl_: *const tll_channel_impl_t,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_impl_get(
        ctx: *const tll_channel_context_t,
        name: *const ::std::os::raw::c_char,
    ) -> *const tll_channel_impl_t;
}
extern "C" {
    pub fn tll_channel_alias_register_url(
        ctx: *mut tll_channel_context_t,
        name: *const ::std::os::raw::c_char,
        cfg: *const tll_config_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_alias_unregister_url(
        ctx: *mut tll_channel_context_t,
        name: *const ::std::os::raw::c_char,
        cfg: *const tll_config_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_alias_register(
        ctx: *mut tll_channel_context_t,
        name: *const ::std::os::raw::c_char,
        url: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_alias_unregister(
        ctx: *mut tll_channel_context_t,
        name: *const ::std::os::raw::c_char,
        url: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_module_load(
        ctx: *mut tll_channel_context_t,
        module: *const ::std::os::raw::c_char,
        symbol: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_module_unload(
        ctx: *mut tll_channel_context_t,
        module: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
pub const TLL_LOGGER_TRACE: tll_logger_level_t = 0;
pub const TLL_LOGGER_DEBUG: tll_logger_level_t = 1;
pub const TLL_LOGGER_INFO: tll_logger_level_t = 2;
pub const TLL_LOGGER_WARNING: tll_logger_level_t = 3;
pub const TLL_LOGGER_ERROR: tll_logger_level_t = 4;
pub const TLL_LOGGER_CRITICAL: tll_logger_level_t = 5;
pub type tll_logger_level_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tll_channel_impl_t {
    pub init: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut tll_channel_t,
            url: *const tll_config_t,
            parent: *mut tll_channel_t,
            ctx: *mut tll_channel_context_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub free: ::std::option::Option<unsafe extern "C" fn(arg1: *mut tll_channel_t)>,
    pub open: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut tll_channel_t,
            arg2: *const tll_config_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub close: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut tll_channel_t,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub process: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut tll_channel_t,
            timeout: ::std::os::raw::c_long,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub post: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut tll_channel_t,
            msg: *const tll_msg_t,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub scheme: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const tll_channel_t,
            arg2: ::std::os::raw::c_int,
        ) -> *const tll_scheme_t,
    >,
    pub name: *const ::std::os::raw::c_char,
    pub prefix: ::std::os::raw::c_int,
    pub data: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_tll_channel_impl_t() {
    assert_eq!(
        ::std::mem::size_of::<tll_channel_impl_t>(),
        80usize,
        concat!("Size of: ", stringify!(tll_channel_impl_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tll_channel_impl_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tll_channel_impl_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_impl_t>())).init as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_impl_t),
            "::",
            stringify!(init)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_impl_t>())).free as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_impl_t),
            "::",
            stringify!(free)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_impl_t>())).open as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_impl_t),
            "::",
            stringify!(open)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_impl_t>())).close as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_impl_t),
            "::",
            stringify!(close)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_impl_t>())).process as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_impl_t),
            "::",
            stringify!(process)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_impl_t>())).post as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_impl_t),
            "::",
            stringify!(post)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_impl_t>())).scheme as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_impl_t),
            "::",
            stringify!(scheme)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_impl_t>())).name as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_impl_t),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_impl_t>())).prefix as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_impl_t),
            "::",
            stringify!(prefix)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_impl_t>())).data as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_impl_t),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
pub struct tll_channel_stat_t {
    pub rx: tll_stat_field_t,
    pub rxb: tll_stat_field_t,
    pub tx: tll_stat_field_t,
    pub txb: tll_stat_field_t,
}
#[test]
fn bindgen_test_layout_tll_channel_stat_t() {
    assert_eq!(
        ::std::mem::size_of::<tll_channel_stat_t>(),
        64usize,
        concat!("Size of: ", stringify!(tll_channel_stat_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tll_channel_stat_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tll_channel_stat_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_stat_t>())).rx as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_stat_t),
            "::",
            stringify!(rx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_stat_t>())).rxb as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_stat_t),
            "::",
            stringify!(rxb)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_stat_t>())).tx as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_stat_t),
            "::",
            stringify!(tx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_stat_t>())).txb as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_stat_t),
            "::",
            stringify!(txb)
        )
    );
}
impl ::std::fmt::Debug for tll_channel_stat_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "tll_channel_stat_t {{  }}")
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tll_channel_callback_pair_t {
    pub cb: tll_channel_callback_t,
    pub user: *mut ::std::os::raw::c_void,
    pub mask: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_tll_channel_callback_pair_t() {
    assert_eq!(
        ::std::mem::size_of::<tll_channel_callback_pair_t>(),
        24usize,
        concat!("Size of: ", stringify!(tll_channel_callback_pair_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tll_channel_callback_pair_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tll_channel_callback_pair_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_callback_pair_t>())).cb as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_callback_pair_t),
            "::",
            stringify!(cb)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tll_channel_callback_pair_t>())).user as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_callback_pair_t),
            "::",
            stringify!(user)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tll_channel_callback_pair_t>())).mask as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_callback_pair_t),
            "::",
            stringify!(mask)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tll_channel_internal_t {
    pub state: tll_state_t,
    pub self_: *mut tll_channel_t,
    pub name: *const ::std::os::raw::c_char,
    pub caps: ::std::os::raw::c_uint,
    pub dcaps: ::std::os::raw::c_uint,
    pub fd: ::std::os::raw::c_int,
    pub config: *mut tll_config_t,
    pub children: *mut tll_channel_list_t,
    pub data_cb_size: ::std::os::raw::c_uint,
    pub data_cb: *mut tll_channel_callback_pair_t,
    pub cb_size: ::std::os::raw::c_uint,
    pub cb: *mut tll_channel_callback_pair_t,
    pub stat: *mut tll_stat_block_t,
}
#[test]
fn bindgen_test_layout_tll_channel_internal_t() {
    assert_eq!(
        ::std::mem::size_of::<tll_channel_internal_t>(),
        96usize,
        concat!("Size of: ", stringify!(tll_channel_internal_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tll_channel_internal_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tll_channel_internal_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_internal_t>())).state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_internal_t),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_internal_t>())).self_ as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_internal_t),
            "::",
            stringify!(self_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_internal_t>())).name as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_internal_t),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_internal_t>())).caps as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_internal_t),
            "::",
            stringify!(caps)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_internal_t>())).dcaps as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_internal_t),
            "::",
            stringify!(dcaps)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_internal_t>())).fd as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_internal_t),
            "::",
            stringify!(fd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_internal_t>())).config as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_internal_t),
            "::",
            stringify!(config)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_internal_t>())).children as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_internal_t),
            "::",
            stringify!(children)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tll_channel_internal_t>())).data_cb_size as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_internal_t),
            "::",
            stringify!(data_cb_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_internal_t>())).data_cb as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_internal_t),
            "::",
            stringify!(data_cb)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_internal_t>())).cb_size as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_internal_t),
            "::",
            stringify!(cb_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_internal_t>())).cb as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_internal_t),
            "::",
            stringify!(cb)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_internal_t>())).stat as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_internal_t),
            "::",
            stringify!(stat)
        )
    );
}
pub const TLL_CHANNEL_MODULE_DLOPEN_GLOBAL: tll_channel_module_flags_t = 1;
pub type tll_channel_module_flags_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tll_channel_module_t {
    pub version: ::std::os::raw::c_int,
    //pub impl_: *mut *mut tll_channel_impl_t,
    pub impl_: i64,
    pub init: ::std::option::Option<
        unsafe extern "C" fn(
            m: *mut tll_channel_module_t,
            ctx: *mut tll_channel_context_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub free: ::std::option::Option<
        unsafe extern "C" fn(
            m: *mut tll_channel_module_t,
            ctx: *mut tll_channel_context_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub flags: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_tll_channel_module_t() {
    assert_eq!(
        ::std::mem::size_of::<tll_channel_module_t>(),
        40usize,
        concat!("Size of: ", stringify!(tll_channel_module_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tll_channel_module_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tll_channel_module_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_module_t>())).version as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_module_t),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_module_t>())).impl_ as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_module_t),
            "::",
            stringify!(impl_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_module_t>())).init as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_module_t),
            "::",
            stringify!(init)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_module_t>())).free as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_module_t),
            "::",
            stringify!(free)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_channel_module_t>())).flags as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_channel_module_t),
            "::",
            stringify!(flags)
        )
    );
}
extern "C" {
    pub fn tll_channel_list_free(l: *mut tll_channel_list_t);
}
extern "C" {
    pub fn tll_channel_list_add(
        l: *mut *mut tll_channel_list_t,
        c: *mut tll_channel_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_list_del(
        l: *mut *mut tll_channel_list_t,
        c: *const tll_channel_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_internal_init(ptr: *mut tll_channel_internal_t);
}
extern "C" {
    pub fn tll_channel_internal_clear(ptr: *mut tll_channel_internal_t);
}
extern "C" {
    pub fn tll_channel_internal_child_add(
        ptr: *mut tll_channel_internal_t,
        c: *mut tll_channel_t,
        tag: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_channel_internal_child_del(
        ptr: *mut tll_channel_internal_t,
        c: *const tll_channel_t,
        tag: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub const TLL_MESSAGE_LOG_DISABLE: tll_channel_log_msg_format_t = 0;
pub const TLL_MESSAGE_LOG_FRAME: tll_channel_log_msg_format_t = 1;
pub const TLL_MESSAGE_LOG_TEXT: tll_channel_log_msg_format_t = 2;
pub const TLL_MESSAGE_LOG_TEXT_HEX: tll_channel_log_msg_format_t = 3;
pub const TLL_MESSAGE_LOG_SCHEME: tll_channel_log_msg_format_t = 4;
pub type tll_channel_log_msg_format_t = ::std::os::raw::c_uint;
extern "C" {
    pub fn tll_channel_log_msg(
        c: *const tll_channel_t,
        log: *const ::std::os::raw::c_char,
        level: tll_logger_level_t,
        format: tll_channel_log_msg_format_t,
        msg: *const tll_msg_t,
        text: *const ::std::os::raw::c_char,
        tlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
