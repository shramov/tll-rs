use tll_sys::logger::*;

use crate::config::Config;
use crate::error::*;

use std::ffi::CStr;
use std::os::raw::{c_int, c_char};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Level {
    Trace = TLL_LOGGER_TRACE as isize,
    Debug = TLL_LOGGER_DEBUG as isize,
    Info = TLL_LOGGER_INFO as isize,
    Warning = TLL_LOGGER_WARNING as isize,
    Error = TLL_LOGGER_ERROR as isize,
    Critical = TLL_LOGGER_CRITICAL as isize,
}

//impl Level {
//    pub const Crit : Level = Level::Critical;
//}

#[derive(Debug, PartialEq, Eq)]
pub struct Logger {
    ptr: * mut tll_logger_t
}

impl Logger {
    pub fn new(name: &str) -> Logger
    {
        let ptr = unsafe { tll_logger_new(name.as_ptr() as *const c_char, name.len() as c_int) };
        assert!(!ptr.is_null());
        Logger { ptr: ptr }
    }

    pub fn name<'a>(&'a self) -> &'a str
    {
        let n = unsafe { tll_logger_name(self.ptr) };
        if n.is_null() { "" } else {
            unsafe { CStr::from_ptr(n) }.to_str().unwrap()
        }
    }

    fn level(&self) -> u32
    {
        unsafe { (*self.ptr).level }
    }

    pub fn enabled(&self, level: Level) -> bool
    {
        self.level() <= level as u32
    }

    pub fn log(&self, level: Level, msg: &str)
    {
        if !self.enabled(level) { return; }
        unsafe { tll_logger_log(self.ptr, level as tll_logger_level_t, msg.as_ptr() as *const c_char, msg.len()) };
    }

    pub fn trace(&self, msg: &str) { self.log(Level::Trace, msg) }
    pub fn debug(&self, msg: &str) { self.log(Level::Debug, msg) }
    pub fn info(&self, msg: &str) { self.log(Level::Info, msg) }
    pub fn warning(&self, msg: &str) { self.log(Level::Warning, msg) }
    pub fn error(&self, msg: &str) { self.log(Level::Error, msg) }
    pub fn critical(&self, msg: &str) { self.log(Level::Critical, msg) }

    pub fn config(cfg: &Config) -> Result<()>
    {
        let r = unsafe { tll_logger_config(cfg.as_ptr()) };
        if r != 0 { return Err(Error::from("Failed to configure logger")) }
        Ok(())
    }

}

impl Drop for Logger {
    fn drop(&mut self) {
        unsafe { tll_logger_free(self.ptr); }
    }
}
