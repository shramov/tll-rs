#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use crate::channel::msg::*;

use std::os::raw::c_int;
use std::os::raw::c_uint;
use std::os::raw::c_long;
use std::os::raw::c_longlong;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::option::Option;

use crate::config::tll_config_t;
use crate::scheme::tll_scheme_t;
use crate::channel::impl_::tll_channel_impl_t;
use crate::channel::impl_::tll_channel_internal_t;

pub type tll_state_t = u32;
pub const TLL_STATE_CLOSED : tll_state_t = 0u32;
pub const TLL_STATE_OPENING : tll_state_t = 1u32;
pub const TLL_STATE_ACTIVE : tll_state_t = 2u32;
pub const TLL_STATE_CLOSING : tll_state_t = 3u32;
pub const TLL_STATE_ERROR : tll_state_t = 4u32;
pub const TLL_STATE_DESTROY : tll_state_t = 5u32;

pub type tll_process_flags_t = u32;
pub const TLL_PROCESS_ONE_LEVEL : tll_process_flags_t = 1;

#[ repr ( C ) ]
#[ derive ( Debug, Copy, Clone ) ]
pub struct tll_channel_context_t { _unused : [ u8; 0 ], }

pub type tll_channel_cap_t = u32;
pub const TLL_CAPS_INPUT : tll_channel_cap_t = 4;
pub const TLL_CAPS_OUTPUT : tll_channel_cap_t = 8;
pub const TLL_CAPS_INOUT : tll_channel_cap_t = 12;
pub const TLL_CAPS_EX_BIT : tll_channel_cap_t = 8388608;
pub const TLL_CAPS_PROXY : tll_channel_cap_t = 8388608;
pub const TLL_CAPS_CUSTOM : tll_channel_cap_t = 8388609;

pub type tll_channel_dcap_t = u32;
pub const TLL_DCAPS_ZERO : tll_channel_dcap_t = 0;
pub const TLL_DCAPS_POLLIN : tll_channel_dcap_t = 1;
pub const TLL_DCAPS_POLLOUT : tll_channel_dcap_t = 2;
pub const TLL_DCAPS_POLLMASK : tll_channel_dcap_t = 3;
pub const TLL_DCAPS_PROCESS : tll_channel_dcap_t = 16;
pub const TLL_DCAPS_PENDING : tll_channel_dcap_t = 32;
pub const TLL_DCAPS_SUSPEND : tll_channel_dcap_t = 64;
pub const TLL_DCAPS_SUSPEND_PERMANENT : tll_channel_dcap_t = 112;

#[ repr ( C ) ]
#[ derive ( Debug, Copy, Clone ) ]
pub struct tll_channel_list_t {
    pub channel : * mut tll_channel_t,
    pub next : * mut tll_channel_list_t,
}

#[ test ]
fn bindgen_test_layout_tll_channel_list_t ()
{
    assert_eq!(::std::mem::size_of::<tll_channel_list_t> (), 16usize, concat!( "Size of: ", stringify!( tll_channel_list_t ) ) );
    assert_eq!(::std::mem::align_of::<tll_channel_list_t> (), 8usize, concat!( "Alignment of ", stringify!( tll_channel_list_t ) ) );
    assert_eq!( unsafe { & ( * (::std::ptr::null::<tll_channel_list_t>()) ) . channel as * const _ as usize }, 0usize, concat!("Offset of field: ", stringify!(tll_channel_list_t), "::", stringify!(channel)));
    assert_eq!( unsafe { & ( * (::std::ptr::null::<tll_channel_list_t>()) ) . next as * const _ as usize }, 8usize, concat!("Offset of field: ", stringify!(tll_channel_list_t), "::", stringify!(next)));
}

#[ repr ( C ) ]
#[ derive ( Debug, Copy, Clone ) ]
pub struct tll_channel_t {
    pub impl_ : * const tll_channel_impl_t,
    pub data : * mut c_void,
    pub internal : * mut tll_channel_internal_t,
    pub context : * mut tll_channel_context_t,
    pub parent : * mut tll_channel_t,
}

#[ test ]
fn bindgen_test_layout_tll_channel_t ()
{
    assert_eq!(::std::mem::size_of::<tll_channel_t> (), 40usize, concat!( "Size of: ", stringify!(tll_channel_t) ) );
    assert_eq!(::std::mem::align_of::<tll_channel_t> (), 8usize, concat!( "Alignment of ", stringify!(tll_channel_t) ) );
    assert_eq!( unsafe { & ( * (::std::ptr::null::<tll_channel_t> () ) ) . impl_ as * const _ as usize }, 0usize, concat!( "Offset of field: ", stringify!(tll_channel_t), "::", stringify!(impl_) ) );
    assert_eq!( unsafe { & ( * (::std::ptr::null::<tll_channel_t> () ) ) . data as * const _ as usize }, 8usize, concat!( "Offset of field: ", stringify!(tll_channel_t), "::", stringify!(data) ) );
    assert_eq!( unsafe { & ( * (::std::ptr::null::<tll_channel_t> () ) ) . internal as * const _ as usize }, 16usize, concat!( "Offset of field: ", stringify!(tll_channel_t), "::", stringify!(internal) ) );
    assert_eq!( unsafe { & ( * (::std::ptr::null::<tll_channel_t> () ) ) . context as * const _ as usize }, 24usize, concat!( "Offset of field: ", stringify!(tll_channel_t), "::", stringify!(context) ) );
    assert_eq!( unsafe { & ( * (::std::ptr::null::<tll_channel_t> () ) ) . parent as * const _ as usize }, 32usize, concat!( "Offset of field: ", stringify!(tll_channel_t), "::", stringify!(parent) ) );
}

pub type tll_channel_callback_t = Option < unsafe extern "C" fn ( c : * const tll_channel_t, msg : * const tll_msg_t, user : * mut c_void ) -> c_int >;

extern "C" {
    pub fn tll_channel_callback_add ( arg1 : * mut tll_channel_t, cb : tll_channel_callback_t, user : * mut c_void, mask : c_uint ) -> c_int;
    pub fn tll_channel_callback_del ( arg1 : * mut tll_channel_t, cb : tll_channel_callback_t, user : * mut c_void, mask : c_uint ) -> c_int;
    pub fn tll_channel_new ( str : * const c_char, len : usize, master : * mut tll_channel_t, ctx : * mut tll_channel_context_t ) -> * mut tll_channel_t;
    pub fn tll_channel_free ( arg1 : * mut tll_channel_t );
    pub fn tll_channel_open ( arg1 : * mut tll_channel_t, str : * const c_char, len : usize ) -> c_int;
    pub fn tll_channel_close ( arg1 : * mut tll_channel_t ) -> c_int;
    pub fn tll_channel_process ( c : * mut tll_channel_t, timeout : c_long, flags : c_int ) -> c_int;
    pub fn tll_channel_post ( c : * mut tll_channel_t, msg : * const tll_msg_t, flags : c_int ) -> c_int;
    pub fn tll_channel_suspend ( c : * mut tll_channel_t ) -> c_int;
    pub fn tll_channel_resume ( c : * mut tll_channel_t ) -> c_int;
    pub fn tll_channel_state ( c : * const tll_channel_t ) -> tll_state_t;
    pub fn tll_channel_name ( c : * const tll_channel_t ) -> * const c_char;
    pub fn tll_channel_caps ( c : * const tll_channel_t ) -> c_longlong;
    pub fn tll_channel_dcaps ( c : * const tll_channel_t ) -> c_longlong;
    pub fn tll_channel_fd ( c : * const tll_channel_t ) -> c_int;
    pub fn tll_channel_context ( c : * const tll_channel_t ) -> * mut tll_channel_context_t;
    pub fn tll_channel_config ( c : * mut tll_channel_t ) -> * mut tll_config_t;
    pub fn tll_channel_children ( c : * mut tll_channel_t ) -> * mut tll_channel_list_t;

    pub fn tll_channel_scheme ( c : * const tll_channel_t, type_ : c_int ) -> * const tll_scheme_t;
    pub fn tll_channel_get ( ctx : * const tll_channel_context_t, name : * const c_char, len : c_int ) -> * mut tll_channel_t;
    pub fn tll_channel_context_new ( defaults : * mut tll_config_t ) -> * mut tll_channel_context_t;
    pub fn tll_channel_context_ref ( arg1 : * mut tll_channel_context_t ) -> * mut tll_channel_context_t;
    pub fn tll_channel_context_default () -> * mut tll_channel_context_t;
    pub fn tll_channel_context_free ( arg1 : * mut tll_channel_context_t );
    pub fn tll_channel_context_config ( arg1 : * mut tll_channel_context_t ) -> * mut tll_config_t;
    pub fn tll_channel_context_config_defaults ( arg1 : * mut tll_channel_context_t ) -> * mut tll_config_t;
    pub fn tll_channel_context_scheme_load ( arg1 : * mut tll_channel_context_t, url : * const c_char, len : c_int, cache : c_int ) -> * const tll_scheme_t;
    pub fn tll_channel_impl_get ( ctx : * const tll_channel_context_t, name : * const c_char ) -> * const tll_channel_impl_t;
    pub fn tll_channel_impl_register ( ctx : * mut tll_channel_context_t, impl_ : * const tll_channel_impl_t, name : * const c_char ) -> c_int;
    pub fn tll_channel_impl_unregister ( ctx : * mut tll_channel_context_t, impl_ : * const tll_channel_impl_t, name : * const c_char ) -> c_int;
    pub fn tll_channel_module_load ( ctx : * mut tll_channel_context_t, module : * const c_char, symbol : * const c_char ) -> c_int;
    pub fn tll_channel_module_unload ( ctx : * mut tll_channel_context_t, module : * const c_char ) -> c_int;
}
