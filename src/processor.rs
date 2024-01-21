use tll_sys::processor_loop::*;

use crate::channel::*;
use crate::config::Config;
use crate::error::*;

use std::os::raw::{c_char, c_int, c_long};

pub struct Loop {
    ptr: *mut tll_processor_loop_t
}

impl Drop for Loop {
    fn drop(&mut self) {
        unsafe { tll_processor_loop_free(self.ptr); }
    }
}

impl Loop {
    pub fn new(name: &str) -> Result<Self>
    {
        let ptr = unsafe { tll_processor_loop_new(name.as_ptr() as *const c_char, name.len() as i32) };
        if ptr.is_null() {
            Err(Error::from("Failed to create new Loop"))
        } else {
            Ok(Loop { ptr: ptr })
        }
    }

    pub fn new_cfg(cfg: &Config) -> Result<Self>
    {
        let ptr = unsafe { tll_processor_loop_new_cfg(cfg.as_ptr()) };
        if ptr.is_null() {
            Err(Error::from("Failed to create new Loop"))
        } else {
            Ok(Loop { ptr: ptr })
        }
    }

    pub fn as_ptr(&self) -> * mut tll_processor_loop_t { self.ptr }

    pub fn add(&mut self, c: &mut Channel) -> Result<()>
    {
        let r = unsafe { tll_processor_loop_add(self.ptr, c.as_ptr()) };
        error_check_fn(r, || format!("Failed to add channel {} into loop", c.name()))
    }

    pub fn del(&mut self, c: &Channel) -> Result<()>
    {
        error_check_fn(unsafe { tll_processor_loop_del(self.ptr, c.as_const_ptr()) }, || format!("Failed to remove channel {} from loop", c.name()))
    }

    pub fn poll(&mut self, timeout: i64) -> Option<Channel>
    {
        let r = unsafe { tll_processor_loop_poll(self.ptr, timeout as c_long) };
        if r.is_null() { None } else { Some( Channel::from_ptr(r) ) }
    }

    pub fn process(&mut self) -> Result<()> { error_check_str(unsafe { tll_processor_loop_process(self.ptr) }, "Loop process failed") }
    pub fn pending(&mut self) -> bool { unsafe { tll_processor_loop_pending(self.ptr) != 0 } }

    pub fn run(&mut self, timeout: i64) -> Result<()> { error_check_str(unsafe { tll_processor_loop_run(self.ptr, timeout as c_long) }, "Loop run failed") }
    pub fn step(&mut self, timeout: i64) -> Result<()> { error_check_str(unsafe { tll_processor_loop_step(self.ptr, timeout as c_long) }, "Loop step failed") }

    pub fn get_stop(&mut self) -> i32 { unsafe { tll_processor_loop_stop_get(self.ptr) } }
    pub fn set_stop(&mut self, stop: i32) -> i32 { unsafe { tll_processor_loop_stop_set(self.ptr, stop as c_int) } }
}
