/* automatically generated by rust-bindgen 0.60.1 */

#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tll_config_t {
    _unused: [u8; 0],
}
pub type tll_config_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        key: *const ::std::os::raw::c_char,
        klen: ::std::os::raw::c_int,
        value: *const tll_config_t,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type tll_config_value_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        len: *mut ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_char,
>;
pub type tll_config_value_callback_free_t = ::std::option::Option<
    unsafe extern "C" fn(cb: tll_config_value_callback_t, data: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn tll_config_has(
        arg1: *const tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_sub(
        arg1: *mut tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
        create: ::std::os::raw::c_int,
    ) -> *mut tll_config_t;
}
extern "C" {
    pub fn tll_config_sub_const(
        arg1: *const tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
    ) -> *const tll_config_t;
}
extern "C" {
    pub fn tll_config_set(
        cfg: *mut tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_char,
        vlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_set_callback(
        arg1: *mut tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
        cb: tll_config_value_callback_t,
        user: *mut ::std::os::raw::c_void,
        deleter: tll_config_value_callback_free_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_set_link(
        arg1: *mut tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
        dest: *const ::std::os::raw::c_char,
        dlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_unset(
        arg1: *mut tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_unlink(
        arg1: *mut tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_remove(
        arg1: *mut tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_del(
        arg1: *mut tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
        recursive: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_set_config(
        arg1: *mut tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
        c: *mut tll_config_t,
        consume: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_merge(
        dest: *mut tll_config_t,
        src: *mut tll_config_t,
        overwrite: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_get(
        arg1: *const tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
        value: *mut ::std::os::raw::c_char,
        vlen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_get_copy(
        arg1: *const tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
        vlen: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tll_config_value_free(value: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn tll_config_value_dup(
        value: *const ::std::os::raw::c_char,
        vlen: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tll_config_get_url(
        arg1: *const tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
    ) -> *mut tll_config_t;
}
extern "C" {
    pub fn tll_config_list(
        arg1: *const tll_config_t,
        cb: tll_config_callback_t,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_browse(
        arg1: *const tll_config_t,
        mask: *const ::std::os::raw::c_char,
        mlen: ::std::os::raw::c_int,
        cb: tll_config_callback_t,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_value(arg1: *const tll_config_t) -> ::std::os::raw::c_int;
}
pub type tll_config_load_t = ::std::option::Option<
    unsafe extern "C" fn(
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> *mut tll_config_t,
>;
extern "C" {
    pub fn tll_config_load_register(
        prefix: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
        cb: tll_config_load_t,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_load_unregister(
        prefix: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
        cb: tll_config_load_t,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_new() -> *mut tll_config_t;
}
extern "C" {
    pub fn tll_config_copy(arg1: *const tll_config_t) -> *mut tll_config_t;
}
extern "C" {
    pub fn tll_config_load(
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
    ) -> *mut tll_config_t;
}
extern "C" {
    pub fn tll_config_load_data(
        proto: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_char,
        dlen: ::std::os::raw::c_int,
    ) -> *mut tll_config_t;
}
extern "C" {
    pub fn tll_config_process_imports(
        arg1: *mut tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tll_config_ref(arg1: *const tll_config_t) -> *const tll_config_t;
}
extern "C" {
    pub fn tll_config_unref(arg1: *const tll_config_t);
}
extern "C" {
    pub fn tll_config_parent(arg1: *const tll_config_t) -> *const tll_config_t;
}
extern "C" {
    pub fn tll_config_root(arg1: *const tll_config_t) -> *const tll_config_t;
}
extern "C" {
    pub fn tll_config_detach(
        arg1: *mut tll_config_t,
        path: *const ::std::os::raw::c_char,
        plen: ::std::os::raw::c_int,
    ) -> *mut tll_config_t;
}
