use tll_sys::config::*;
use std::option::Option;
use std::os::raw::{c_int, c_char};

pub struct Config {
    ptr: *mut tll_config_t
}

impl From<* mut tll_config_t> for Config {
    fn from(ptr: * mut tll_config_t) -> Self
    {
        assert!(!ptr.is_null());
        unsafe { tll_config_ref(ptr) };
        Config { ptr: ptr }
    }

}

impl Config {
    pub fn new() -> Config
    {
        let ptr = unsafe { tll_config_new() };
        assert!(!ptr.is_null());
        Config { ptr: ptr }
    }

    pub fn consume(ptr: * mut tll_config_t) -> Self
    {
        assert!(!ptr.is_null());
        Config { ptr: ptr }
    }

    pub fn load(url: &str) -> Option<Config>
    {
        let ptr = unsafe { tll_config_load(url.as_ptr() as *const c_char, url.len() as c_int) };
        if ptr.is_null() { None } else { Some(Config {ptr: ptr}) }
    }

    pub fn as_ptr(&self) -> *const tll_config_t { self.ptr }
    pub fn as_mut_ptr(&mut self) -> *mut tll_config_t { self.ptr }

    pub fn get(&self, key: &str) -> Option<String>
    {
        let mut len = 0 as c_int;
        let ptr = unsafe { tll_config_get_copy(self.ptr, key.as_ptr() as *const c_char, key.len() as c_int, &mut len as *mut c_int) };
        if ptr.is_null() {
            return None;
        } else {
            let v = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
            let s = std::str::from_utf8(v).unwrap().to_owned();
            unsafe { tll_config_value_free(ptr) };
            Some(s)
        }
    }

    pub fn set(&self, key: &str, value: &str)
    {
        unsafe { tll_config_set(self.ptr, key.as_ptr() as *const c_char, key.len() as c_int, value.as_ptr() as *const c_char, value.len() as c_int) };
    }

    pub fn sub(&self, key: &str) -> Option<Config>
    {
        let ptr = unsafe { tll_config_sub(self.ptr, key.as_ptr() as *const c_char, key.len() as c_int) };
        if !ptr.is_null() {
            return Some (Config { ptr: ptr })
        } else {
            return None;
        }
    }

}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe { tll_config_unref(self.ptr); }
    }
}
