use std::option::Option;
use std::os::raw::{c_char, c_int, c_void};
use std::str::FromStr;
use tll_sys::config::*;

use crate::result::*;

pub trait FromStrCustom: Sized {
    fn from_str_custom(s: &str) -> Result<Self>;
}

impl FromStrCustom for bool {
    fn from_str_custom(s: &str) -> Result<Self> {
        match s {
            "true" | "yes" | "1" => Ok(true),
            "false" | "no" | "0" => Ok(false),
            _ => Err(Error::from("invalid bool value")),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    ptr: *mut tll_config_t,
}

impl From<*mut tll_config_t> for Config {
    fn from(ptr: *mut tll_config_t) -> Self {
        assert!(!ptr.is_null());
        unsafe { tll_config_ref(ptr) };
        Config { ptr: ptr }
    }
}

impl Clone for Config {
    fn clone(&self) -> Self {
        Config::from(self.as_ptr() as *mut tll_config_t)
    }
}

type BrowseVec = Vec<(String, Config)>;
extern "C" fn browse_cb(key: *const c_char, klen: c_int, cfg: *const tll_config_t, user: *mut c_void) -> c_int {
    let r = unsafe { &mut *(user as *mut BrowseVec) };
    let v = unsafe { std::slice::from_raw_parts(key as *const u8, klen as usize) };
    if let Ok(s) = String::from_utf8(v.to_vec()) {
        r.push((s, Config::from(cfg as *mut tll_config_t)));
        return 0;
    }
    return EINVAL;
}

fn value_to_option(ptr: *const c_char, len: c_int) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    let v = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let r = String::from_utf8(v.to_vec());
    unsafe { tll_config_value_free(ptr) };
    r.ok()
}

impl Config {
    pub fn new() -> Config {
        let ptr = unsafe { tll_config_new() };
        assert!(!ptr.is_null());
        Config { ptr: ptr }
    }

    pub fn consume(ptr: *mut tll_config_t) -> Self {
        assert!(!ptr.is_null());
        Config { ptr: ptr }
    }

    pub fn from_values(values: &[(&str, &str)]) -> Self {
        let mut cfg = Self::new();
        for (k, v) in values {
            cfg.set(k, v);
        }
        cfg
    }

    pub fn load(url: &str) -> Result<Config> {
        let ptr = unsafe { tll_config_load(url.as_ptr() as *const c_char, url.len() as c_int) };
        if ptr.is_null() {
            Err(Error::from("Invalid config data"))
        } else {
            Ok(Config { ptr: ptr })
        }
    }

    pub fn load_data(proto: &str, data: &str) -> Result<Config> {
        let ptr = unsafe {
            tll_config_load_data(
                proto.as_ptr() as *const c_char,
                proto.len() as c_int,
                data.as_ptr() as *const c_char,
                data.len() as c_int,
            )
        };
        if ptr.is_null() {
            Err(Error::from("Invalid config data"))
        } else {
            Ok(Config { ptr: ptr })
        }
    }

    pub fn copy(&self) -> Self {
        let ptr = unsafe { tll_config_copy(self.ptr) }; // Can only fail if self.ptr is null
        Config::consume(ptr)
    }

    pub fn as_ptr(&self) -> *const tll_config_t {
        self.ptr
    }
    pub fn as_mut_ptr(&mut self) -> *mut tll_config_t {
        self.ptr
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let mut len = 0 as c_int;
        let ptr = unsafe {
            tll_config_get_copy(
                self.ptr,
                key.as_ptr() as *const c_char,
                key.len() as c_int,
                &mut len as *mut c_int,
            )
        };
        value_to_option(ptr, len)
    }

    pub fn get_self(&self) -> Option<String> {
        let mut len = 0 as c_int;
        let ptr = unsafe { tll_config_get_copy(self.ptr, std::ptr::null(), 0, &mut len as *mut c_int) };
        value_to_option(ptr, len)
    }

    pub fn get_typed<T: FromStr>(&self, key: &str, default: T) -> Result<T>
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        match self.get(key) {
            Some(s) => {
                if s.len() == 0 {
                    return Ok(default);
                }
                T::from_str(&s).map_err(|e| Error::from(format!("{:?}", e).as_str()))
            }
            None => Ok(default),
        }
    }

    pub fn get_bool(&self, key: &str, default: bool) -> Result<bool> {
        match self.get(key) {
            Some(s) => {
                if s.len() == 0 {
                    return Ok(default);
                }
                bool::from_str_custom(&s)
            }
            None => Ok(default),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> &mut Self {
        unsafe {
            tll_config_set(
                self.ptr,
                key.as_ptr() as *const c_char,
                key.len() as c_int,
                value.as_ptr() as *const c_char,
                value.len() as c_int,
            )
        };
        self
    }

    pub fn remove(&self, key: &str) {
        unsafe { tll_config_remove(self.ptr, key.as_ptr() as *const c_char, key.len() as c_int) };
    }

    pub fn unlink(&self, key: &str) {
        unsafe { tll_config_unlink(self.ptr, key.as_ptr() as *const c_char, key.len() as c_int) };
    }

    pub fn sub(&self, key: &str) -> Option<Config> {
        let ptr = unsafe { tll_config_sub(self.ptr, key.as_ptr() as *const c_char, key.len() as c_int, 0 as c_int) };
        if !ptr.is_null() {
            return Some(Config { ptr: ptr });
        } else {
            return None;
        }
    }

    pub fn browse(&self, mask: &str) -> Vec<(String, Config)> {
        let mut r = BrowseVec::new();
        unsafe {
            tll_config_browse(
                self.ptr,
                mask.as_ptr() as *const c_char,
                mask.len() as c_int,
                Some(browse_cb),
                &mut r as *mut BrowseVec as *mut c_void,
            )
        };
        return r;
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe {
            tll_config_unref(self.ptr);
        }
    }
}

#[derive(Default)]
pub struct ConfigChain {
    chain: [Option<Config>; 3],
}

impl ConfigChain {
    pub fn new(c0: Option<Config>, c1: Option<Config>, c2: Option<Config>) -> Self {
        Self { chain: [c0, c1, c2] }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let mut r = Option::<String>::default();
        for oc in &self.chain {
            if let Some(c) = oc {
                if let Some(s) = c.get(key) {
                    if s.len() == 0 {
                        r = Some(s);
                    } else {
                        return Some(s);
                    }
                }
            }
        }
        r
    }

    pub fn get_typed<T: FromStr>(&self, key: &str, default: T) -> Result<T>
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        if let Some(s) = self.get(key) {
            if s.len() != 0 {
                return T::from_str(&s).map_err(|e| Error::from(format!("invalid '{}' value '{}': {:?}", key, s, e)));
            }
        }
        Ok(default)
    }

    pub fn get_typed_custom<T: FromStrCustom>(&self, key: &str, default: T) -> Result<T> {
        if let Some(s) = self.get(key) {
            if s.len() != 0 {
                return T::from_str_custom(&s)
                    .map_err(|e| Error::from(format!("invalid '{}' value '{}': {}", key, s, e)));
            }
        }
        Ok(default)
    }

    pub fn get_bool(&self, key: &str, default: bool) -> Result<bool> {
        self.get_typed_custom(key, default)
    }
}

pub trait ConfigChainBuilder {
    fn config_chain(&self, cfg: &Config) -> ConfigChain;
}
