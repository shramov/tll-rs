use tll_sys::config::*;
use std::option::Option;
use std::str::FromStr;
use std::os::raw::{c_int, c_char};

use crate::error::*;

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

    pub fn load(url: &str) -> Result<Config>
    {
        let ptr = unsafe { tll_config_load(url.as_ptr() as *const c_char, url.len() as c_int) };
        if ptr.is_null() { Err(Error::from("Invalid config data")) } else { Ok(Config {ptr: ptr}) }
    }

    pub fn load_data(proto: &str, data: &str) -> Result<Config>
    {
        let ptr = unsafe { tll_config_load_data(proto.as_ptr() as *const c_char, proto.len() as c_int, data.as_ptr() as *const c_char, data.len() as c_int) };
        if ptr.is_null() { Err(Error::from("Invalid config data")) } else { Ok(Config {ptr: ptr}) }
    }

    pub fn copy(&self) -> Self
    {
        let ptr = unsafe { tll_config_copy(self.ptr) }; // Can only fail if self.ptr is null
        Config::consume(ptr)
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

    pub fn get_typed<T : FromStr>(&self, key: &str, default: T) -> Result<T> where <T as FromStr>::Err : std::fmt::Debug
    {
        match self.get(key) {
            Some(s) => {
                if s.len() == 0 { return Ok(default); }
                T::from_str(&s).map_err(|e| Error::from(format!("{:?}", e).as_str()))
            },
            None => Ok(default)
        }
    }

    pub fn set(&self, key: &str, value: &str)
    {
        unsafe { tll_config_set(self.ptr, key.as_ptr() as *const c_char, key.len() as c_int, value.as_ptr() as *const c_char, value.len() as c_int) };
    }

    pub fn remove(&self, key: &str)
    {
        unsafe { tll_config_del(self.ptr, key.as_ptr() as *const c_char, key.len() as c_int, 0 as c_int) };
    }

    pub fn remove_rec(&self, key: &str)
    {
        unsafe { tll_config_del(self.ptr, key.as_ptr() as *const c_char, key.len() as c_int, 1 as c_int) };
    }

    pub fn sub(&self, key: &str) -> Option<Config>
    {
        let ptr = unsafe { tll_config_sub(self.ptr, key.as_ptr() as *const c_char, key.len() as c_int, 0 as c_int) };
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
