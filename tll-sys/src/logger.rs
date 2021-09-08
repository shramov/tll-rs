/* automatically generated by rust-bindgen 0.59.1 */

#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]
use crate::config::tll_config_t;

pub const TLL_LOGGER_TRACE: tll_logger_level_t = 0;
pub const TLL_LOGGER_DEBUG: tll_logger_level_t = 1;
pub const TLL_LOGGER_INFO: tll_logger_level_t = 2;
pub const TLL_LOGGER_WARNING: tll_logger_level_t = 3;
pub const TLL_LOGGER_ERROR: tll_logger_level_t = 4;
pub const TLL_LOGGER_CRITICAL: tll_logger_level_t = 5;
pub type tll_logger_level_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tll_logger_t {
    pub level: tll_logger_level_t,
}
#[test]
fn bindgen_test_layout_tll_logger_t() {
    assert_eq!(
        ::std::mem::size_of::<tll_logger_t>(),
        4usize,
        concat!("Size of: ", stringify!(tll_logger_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tll_logger_t>(),
        4usize,
        concat!("Alignment of ", stringify!(tll_logger_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_logger_t>())).level as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_logger_t),
            "::",
            stringify!(level)
        )
    );
}
extern "C" {
    pub fn tll_logger_new(
        name: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> *mut tll_logger_t;
}
extern "C" {
    pub fn tll_logger_free(log: *mut tll_logger_t);
}
extern "C" {
    pub fn tll_logger_config(cfg: *mut tll_config_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_logger_set(
        name: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        level: tll_logger_level_t,
        subtree: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_logger_name(log: *const tll_logger_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn tll_logger_log(
        log: *mut tll_logger_t,
        lvl: tll_logger_level_t,
        buf: *const ::std::os::raw::c_char,
        size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_logger_printf(
        log: *mut tll_logger_t,
        lvl: tll_logger_level_t,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tll_logger_buf_t {
    pub data: *mut ::std::os::raw::c_char,
    pub size: usize,
    pub reserve: usize,
}
#[test]
fn bindgen_test_layout_tll_logger_buf_t() {
    assert_eq!(
        ::std::mem::size_of::<tll_logger_buf_t>(),
        24usize,
        concat!("Size of: ", stringify!(tll_logger_buf_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tll_logger_buf_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tll_logger_buf_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_logger_buf_t>())).data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_logger_buf_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_logger_buf_t>())).size as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_logger_buf_t),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tll_logger_buf_t>())).reserve as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tll_logger_buf_t),
            "::",
            stringify!(reserve)
        )
    );
}
extern "C" {
    pub fn tll_logger_tls_buf() -> *mut tll_logger_buf_t;
}
