use tll_sys::logger::*;

use crate::config::Config;
use crate::result::*;

use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Level {
    Trace = TLL_LOGGER_TRACE as isize,
    Debug = TLL_LOGGER_DEBUG as isize,
    Info = TLL_LOGGER_INFO as isize,
    Warning = TLL_LOGGER_WARNING as isize,
    Error = TLL_LOGGER_ERROR as isize,
    Critical = TLL_LOGGER_CRITICAL as isize,
}

impl From<u32> for Level {
    fn from(v: u32) -> Self {
        match v {
            TLL_LOGGER_TRACE => Level::Trace,
            TLL_LOGGER_DEBUG => Level::Debug,
            TLL_LOGGER_INFO => Level::Info,
            TLL_LOGGER_WARNING => Level::Warning,
            TLL_LOGGER_ERROR => Level::Error,
            TLL_LOGGER_CRITICAL => Level::Critical,
            _ => Level::Critical,
        }
    }
}

pub trait IntoLogger {
    fn into_logger(&self) -> &Logger;
}

#[inline(always)]
pub fn into_logger<T: IntoLogger>(v: &T) -> &Logger {
    v.into_logger()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Logger {
    ptr: *mut tll_logger_t,
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        let ptr = unsafe { tll_logger_copy(self.ptr) };
        assert!(!ptr.is_null());
        Logger { ptr: ptr }
    }
}

impl IntoLogger for Logger {
    fn into_logger(&self) -> &Logger {
        self
    }
}

impl Logger {
    pub fn new(name: &str) -> Logger {
        let ptr = unsafe { tll_logger_new(name.as_ptr() as *const c_char, name.len() as c_int) };
        assert!(!ptr.is_null());
        Logger { ptr: ptr }
    }

    pub fn as_ptr(&self) -> *mut tll_logger_t {
        self.ptr
    }

    pub fn name<'a>(&'a self) -> &'a str {
        let n = unsafe { tll_logger_name(self.ptr) };
        if n.is_null() {
            ""
        } else {
            unsafe { CStr::from_ptr(n) }.to_str().unwrap()
        }
    }

    #[inline(always)]
    fn level_raw(&self) -> u32 {
        unsafe { (*self.ptr).level }
    }

    pub fn level(&self) -> Level {
        Level::from(self.level_raw())
    }

    pub fn set_level(&self, level: Level) {
        unsafe { (*self.ptr).level = level as u32 }
    }

    #[inline(always)]
    pub fn enabled(&self, level: Level) -> bool {
        self.level_raw() <= level as u32
    }

    #[inline(always)]
    pub fn log(&self, level: Level, msg: &str) {
        if !self.enabled(level) {
            return;
        }
        unsafe {
            tll_logger_log(
                self.ptr,
                level as tll_logger_level_t,
                msg.as_ptr() as *const c_char,
                msg.len(),
            )
        };
    }

    #[inline(always)]
    pub fn trace(&self, msg: &str) {
        self.log(Level::Trace, msg)
    }
    #[inline(always)]
    pub fn debug(&self, msg: &str) {
        self.log(Level::Debug, msg)
    }
    #[inline(always)]
    pub fn info(&self, msg: &str) {
        self.log(Level::Info, msg)
    }
    #[inline(always)]
    pub fn warning(&self, msg: &str) {
        self.log(Level::Warning, msg)
    }
    #[inline(always)]
    pub fn error(&self, msg: &str) {
        self.log(Level::Error, msg)
    }
    #[inline(always)]
    pub fn critical(&self, msg: &str) {
        self.log(Level::Critical, msg)
    }

    #[inline(always)]
    pub fn fail<T>(&self, err: T, msg: &str) -> T {
        self.error(msg);
        err
    }

    pub fn config(cfg: &Config) -> Result<()> {
        let r = unsafe { tll_logger_config(cfg.as_ptr()) };
        if r != 0 {
            return Err(Error::from("Failed to configure logger"));
        }
        Ok(())
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        unsafe {
            tll_logger_free(self.ptr);
        }
    }
}

#[macro_export]
//#[clippy::format_args] // See https://github.com/rust-lang/rust/issues/74087
macro_rules! log {
    ($logger:expr, $level:expr, $($arg:tt)+) => ({
        let l = $crate::logger::into_logger($logger);
        if l.enabled($level) {
            l.log($level, &std::format_args!($($arg)+).to_string());
        }
    })
}

#[macro_export]
#[clippy::format_args]
macro_rules! ciritcal { ($logger:expr, $($arg:tt)+) => ($crate::log!($logger, $crate::logger::Level::Critical, $($arg)+)) }

#[macro_export]
#[clippy::format_args]
macro_rules! error { ($logger:expr, $($arg:tt)+) => ($crate::log!($logger, $crate::logger::Level::Error, $($arg)+)) }

#[macro_export]
#[clippy::format_args]
macro_rules! warning { ($logger:expr, $($arg:tt)+) => ($crate::log!($logger, $crate::logger::Level::Warning, $($arg)+)) }

#[macro_export]
#[clippy::format_args]
macro_rules! info { ($logger:expr, $($arg:tt)+) => ($crate::log!($logger, $crate::logger::Level::Info, $($arg)+)) }

#[macro_export]
#[clippy::format_args]
macro_rules! debug { ($logger:expr, $($arg:tt)+) => ($crate::log!($logger, $crate::logger::Level::Debug, $($arg)+)) }

#[macro_export]
#[clippy::format_args]
macro_rules! trace { ($logger:expr, $($arg:tt)+) => ($crate::log!($logger, $crate::logger::Level::Trace, $($arg)+)) }
