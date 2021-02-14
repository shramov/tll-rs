#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use std::os::raw::c_short;
use std::os::raw::c_int;
use std::os::raw::c_longlong;
use std::os::raw::c_void;

pub type tll_msg_type_t = c_short;
pub const TLL_MESSAGE_DATA : tll_msg_type_t = 0;
pub const TLL_MESSAGE_CONTROL : tll_msg_type_t = 1;
pub const TLL_MESSAGE_STATE : tll_msg_type_t = 2;
pub const TLL_MESSAGE_CHANNEL : tll_msg_type_t = 3;

pub type tll_msg_channel_t = u32;
pub const TLL_MESSAGE_CHANNEL_UPDATE : tll_msg_channel_t = 0u32;
pub const TLL_MESSAGE_CHANNEL_ADD : tll_msg_channel_t = 1u32;
pub const TLL_MESSAGE_CHANNEL_DELETE : tll_msg_channel_t = 2u32;

pub type tll_message_mask_t = u32;
pub const TLL_MESSAGE_MASK_ALL : tll_message_mask_t = 0xffffffffu32;
pub const TLL_MESSAGE_MASK_DATA : tll_message_mask_t = 1u32;
pub const TLL_MESSAGE_MASK_CONTROL : tll_message_mask_t = 2u32;
pub const TLL_MESSAGE_MASK_STATE : tll_message_mask_t = 4u32;
pub const TLL_MESSAGE_MASK_CHANNEL : tll_message_mask_t = 8u32;

pub type tll_addr_t = i64;

#[ repr ( C ) ]
#[ derive ( Debug, Copy, Clone ) ]
pub struct tll_msg_t {
    pub type_ : tll_msg_type_t, // c_short
    pub msgid : c_int,
    pub seq : c_longlong,
    pub flags : c_short,
    pub data : * const c_void,
    pub size : usize,
    pub addr : tll_addr_t,
}

#[ test ]
fn bindgen_test_layout_tll_msg_t () {
    assert_eq!(::std::mem::size_of::< tll_msg_t > (), 48usize, concat!( "Size of: ", stringify!( tll_msg_t ) ) );
    assert_eq!(::std::mem::align_of::< tll_msg_t > (), 8usize, concat!( "Alignment of ", stringify!( tll_msg_t ) ) );
    assert_eq!( unsafe { & ( * (::std::ptr::null::<tll_msg_t>())) . type_ as * const _ as usize }, 0usize, concat!( "Offset of field: ", stringify!( tll_msg_t ), "::", stringify!( type_ ) ) );
    assert_eq!( unsafe { & ( * (::std::ptr::null::<tll_msg_t>())) . msgid as * const _ as usize }, 4usize, concat!( "Offset of field: ", stringify!( tll_msg_t ), "::", stringify!( msgid ) ) );
    assert_eq!( unsafe { & ( * (::std::ptr::null::<tll_msg_t>())) . seq as * const _ as usize }, 8usize, concat!( "Offset of field: ", stringify!( tll_msg_t ), "::", stringify!( seq ) ) );
    assert_eq!( unsafe { & ( * (::std::ptr::null::<tll_msg_t>())) . flags as * const _ as usize }, 16usize, concat!( "Offset of field: ", stringify!( tll_msg_t ), "::", stringify!( flags ) ) );
    assert_eq!( unsafe { & ( * (::std::ptr::null::<tll_msg_t>())) . data as * const _ as usize }, 24usize, concat!( "Offset of field: ", stringify!( tll_msg_t ), "::", stringify!( data ) ) );
    assert_eq!( unsafe { & ( * (::std::ptr::null::<tll_msg_t>())) . size as * const _ as usize }, 32usize, concat!( "Offset of field: ", stringify!( tll_msg_t ), "::", stringify!( size ) ) );
    assert_eq!( unsafe { & ( * (::std::ptr::null::<tll_msg_t>())) . addr as * const _ as usize }, 40usize, concat!( "Offset of field: ", stringify!( tll_msg_t ), "::", stringify!( addr ) ) );
}
