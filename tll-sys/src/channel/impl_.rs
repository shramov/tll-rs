#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use std::os::raw::c_int;
use std::os::raw::c_uint;
use std::os::raw::c_long;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::option::Option;

#[allow(unused_imports)]
use std::ptr::null;

use crate::config::tll_config_t;
use crate::scheme::tll_scheme_t;
use crate::channel::channel::*;
use crate::channel::msg::*;

#[ repr ( C ) ]
#[ derive ( Debug, Copy, Clone ) ]
pub struct tll_channel_impl_t {
    pub init : Option < unsafe extern "C" fn ( arg1 : * mut tll_channel_t, str : * const c_char, len : usize, master : * mut tll_channel_t, ctx : * mut tll_channel_context_t ) -> c_int >,
    pub free : Option < unsafe extern "C" fn ( arg1 : * mut tll_channel_t ) >,
    pub open : Option < unsafe extern "C" fn ( arg1 : * mut tll_channel_t, str : * const c_char, len : usize ) -> c_int >,
    pub close : Option < unsafe extern "C" fn ( arg1 : * mut tll_channel_t ) -> c_int >,
    pub process : Option < unsafe extern "C" fn ( arg1 : * mut tll_channel_t, timeout : c_long, flags : c_int ) -> c_int >,
    pub post : Option < unsafe extern "C" fn ( arg1 : * mut tll_channel_t, msg : * const tll_msg_t, flags : c_int ) -> c_int >,
    pub scheme : Option < unsafe extern "C" fn ( arg1 : * const tll_channel_t, arg2 : c_int ) -> * const tll_scheme_t >,
    pub name : * const c_char,
    pub prefix : c_int,
    pub data : * mut c_void,
}

#[ test ]
fn bindgen_test_layout_tll_channel_impl_t(){ assert_eq!(::std::mem::size_of::< tll_channel_impl_t >(), 80usize, concat!( "Size of: ", stringify!( tll_channel_impl_t ) ) );
    assert_eq!(::std::mem::align_of::< tll_channel_impl_t >(), 8usize, concat!( "Alignment of ", stringify!( tll_channel_impl_t ) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_impl_t >()) ) . init as * const _ as usize }, 0usize, concat!( "Offset of field: ", stringify!( tll_channel_impl_t ), "::", stringify!( init ) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_impl_t >()) ) . free as * const _ as usize }, 8usize, concat!( "Offset of field: ", stringify!( tll_channel_impl_t ), "::", stringify!( free ) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_impl_t >()) ) . open as * const _ as usize }, 16usize, concat!( "Offset of field: ", stringify!( tll_channel_impl_t ), "::", stringify!( open ) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_impl_t >()) ) . close as * const _ as usize }, 24usize, concat!( "Offset of field: ", stringify!( tll_channel_impl_t ), "::", stringify!( close ) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_impl_t >()) ) . process as * const _ as usize }, 32usize, concat!( "Offset of field: ", stringify!( tll_channel_impl_t ), "::", stringify!( process ) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_impl_t >()) ) . post as * const _ as usize }, 40usize, concat!( "Offset of field: ", stringify!( tll_channel_impl_t ), "::", stringify!( post ) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_impl_t >()) ) . scheme as * const _ as usize }, 48usize, concat!( "Offset of field: ", stringify!( tll_channel_impl_t ), "::", stringify!( scheme ) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_impl_t >()) ) . name as * const _ as usize }, 56usize, concat!( "Offset of field: ", stringify!( tll_channel_impl_t ), "::", stringify!( name ) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_impl_t >()) ) . prefix as * const _ as usize }, 64usize, concat!( "Offset of field: ", stringify!( tll_channel_impl_t ), "::", stringify!( prefix ) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_impl_t >()) ) . data as * const _ as usize }, 72usize, concat!( "Offset of field: ", stringify!( tll_channel_impl_t ), "::", stringify!( data ) ) );
}

#[ repr ( C ) ]
#[ derive ( Debug, Copy, Clone ) ]
pub struct tll_channel_callback_pair_t { pub cb : tll_channel_callback_t, pub user : * mut c_void, pub mask : c_uint, }

#[ test ]
fn bindgen_test_layout_tll_channel_callback_pair_t(){ assert_eq!(::std::mem::size_of::< tll_channel_callback_pair_t >(), 24usize, concat!( "Size of: ", stringify!( tll_channel_callback_pair_t ) ) );
    assert_eq!(::std::mem::align_of::< tll_channel_callback_pair_t >(), 8usize, concat!( "Alignment of ", stringify!( tll_channel_callback_pair_t ) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_callback_pair_t >()) ) . cb as * const _ as usize }, 0usize, concat!( "Offset of field: ", stringify!( tll_channel_callback_pair_t ), "::", stringify!( cb ) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_callback_pair_t >()) ) . user as * const _ as usize }, 8usize, concat!( "Offset of field: ", stringify!( tll_channel_callback_pair_t ), "::", stringify!( user ) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_callback_pair_t >()) ) . mask as * const _ as usize }, 16usize, concat!( "Offset of field: ", stringify!( tll_channel_callback_pair_t ), "::", stringify!( mask ) ) );
}

#[ repr ( C ) ]
#[ derive ( Debug, Copy, Clone ) ]
pub struct tll_channel_internal_t {
    pub state : tll_state_t,
    pub self_ : * mut tll_channel_t,
    pub name : * const c_char,
    pub caps : c_uint,
    pub dcaps : c_uint,
    pub fd : c_int,
    pub config : * mut tll_config_t,
    pub children : * mut tll_channel_list_t,
    pub data_cb_size : c_uint,
    pub data_cb : * mut tll_channel_callback_pair_t,
    pub cb_size : c_uint,
    pub cb : * mut tll_channel_callback_pair_t,
}

#[ test ]
fn bindgen_test_layout_tll_channel_internal_t(){ assert_eq!(::std::mem::size_of::<tll_channel_internal_t>(), 88usize, concat!( "Size of: ", stringify!(tll_channel_internal_t) ) );
    assert_eq!(::std::mem::align_of::<tll_channel_internal_t>(), 8usize, concat!( "Alignment of ", stringify!(tll_channel_internal_t) ) );
    assert_eq!( unsafe { & ( * (null::<tll_channel_internal_t>()) ) . state as * const _ as usize }, 0usize, concat!( "Offset of field: ", stringify!(tll_channel_internal_t), "::", stringify!( state ) ) );
    assert_eq!( unsafe { & ( * (null::<tll_channel_internal_t>()) ) . self_ as * const _ as usize }, 8usize, concat!( "Offset of field: ", stringify!(tll_channel_internal_t), "::", stringify!( self_ ) ) );
    assert_eq!( unsafe { & ( * (null::<tll_channel_internal_t>()) ) . name as * const _ as usize }, 16usize, concat!( "Offset of field: ", stringify!(tll_channel_internal_t), "::", stringify!( name ) ) );
    assert_eq!( unsafe { & ( * (null::<tll_channel_internal_t>()) ) . caps as * const _ as usize }, 24usize, concat!( "Offset of field: ", stringify!(tll_channel_internal_t), "::", stringify!( caps ) ) );
    assert_eq!( unsafe { & ( * (null::<tll_channel_internal_t>()) ) . dcaps as * const _ as usize }, 28usize, concat!( "Offset of field: ", stringify!(tll_channel_internal_t), "::", stringify!( dcaps ) ) );
    assert_eq!( unsafe { & ( * (null::<tll_channel_internal_t>()) ) . fd as * const _ as usize }, 32usize, concat!( "Offset of field: ", stringify!(tll_channel_internal_t), "::", stringify!( fd ) ) );
    assert_eq!( unsafe { & ( * (null::<tll_channel_internal_t>()) ) . config as * const _ as usize }, 40usize, concat!( "Offset of field: ", stringify!(tll_channel_internal_t), "::", stringify!( config ) ) );
    assert_eq!( unsafe { & ( * (null::<tll_channel_internal_t>()) ) . children as * const _ as usize }, 48usize, concat!( "Offset of field: ", stringify!(tll_channel_internal_t), "::", stringify!( children ) ) );
    assert_eq!( unsafe { & ( * (null::<tll_channel_internal_t>()) ) . data_cb_size as * const _ as usize }, 56usize, concat!( "Offset of field: ", stringify!(tll_channel_internal_t), "::", stringify!( data_cb_size ) ) );
    assert_eq!( unsafe { & ( * (null::<tll_channel_internal_t>()) ) . data_cb as * const _ as usize }, 64usize, concat!( "Offset of field: ", stringify!(tll_channel_internal_t), "::", stringify!( data_cb ) ) );
    assert_eq!( unsafe { & ( * (null::<tll_channel_internal_t>()) ) . cb_size as * const _ as usize }, 72usize, concat!( "Offset of field: ", stringify!(tll_channel_internal_t), "::", stringify!( cb_size ) ) );
    assert_eq!( unsafe { & ( * (null::<tll_channel_internal_t>()) ) . cb as * const _ as usize }, 80usize, concat!( "Offset of field: ", stringify!(tll_channel_internal_t), "::", stringify!( cb ) ) );
}

#[ repr ( C ) ]
#[ derive ( Debug, Copy, Clone ) ]
pub struct tll_channel_module_t {
    pub init : Option < unsafe extern "C" fn ( ctx : * mut tll_channel_context_t ) -> c_int >,
    pub fini : Option < unsafe extern "C" fn ( ctx : * mut tll_channel_context_t ) -> c_int >,
}

#[ test ]
fn bindgen_test_layout_tll_channel_module_t(){
    assert_eq!(::std::mem::size_of::< tll_channel_module_t >(), 16usize, concat!( "Size of: ", stringify!( tll_channel_module_t ) ) );
    assert_eq!(::std::mem::align_of::< tll_channel_module_t >(), 8usize, concat!( "Alignment of ", stringify!( tll_channel_module_t ) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_module_t >()) ) . init as * const _ as usize }, 0usize, concat!( "Offset of field: ", stringify!(tll_channel_module_t), "::", stringify!(init) ) );
    assert_eq!( unsafe { & ( * (null::< tll_channel_module_t >()) ) . fini as * const _ as usize }, 8usize, concat!( "Offset of field: ", stringify!(tll_channel_module_t), "::", stringify!(fini) ) );
}

extern "C" {
    pub fn tll_channel_list_free ( l : * mut tll_channel_list_t );
    pub fn tll_channel_list_add ( l : * mut * mut tll_channel_list_t, c : * mut tll_channel_t ) -> c_int;
    pub fn tll_channel_internal_init ( ptr : * mut tll_channel_internal_t);
    pub fn tll_channel_internal_clear ( ptr : * mut tll_channel_internal_t);
}
