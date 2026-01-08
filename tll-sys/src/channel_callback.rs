use crate::channel::*;
use crate::logger::TLL_LOGGER_INFO;

use std::os::raw::c_char;

#[inline(always)]
fn dump(internal: &tll_channel_internal_t, msg: *const tll_msg_t) {
    if internal.dump != 0 {
        unsafe {
            tll_channel_log_msg(
                internal.self_,
                std::ptr::null(),
                TLL_LOGGER_INFO,
                internal.dump,
                msg,
                "Recv".as_ptr() as *const c_char,
                4,
            )
        };
    }
}

pub fn channel_callback_data(internal: &tll_channel_internal_t, msg: *const tll_msg_t) -> i32 {
    dump(internal, msg);
    for i in 0..internal.data_cb_size {
        unsafe {
            let ptr = internal.data_cb.offset(i as isize);
            let cb = std::ptr::addr_of!((*ptr).cb).read_unaligned();
            let user = std::ptr::addr_of!((*ptr).user).read_unaligned();
            cb.unwrap()(internal.self_, msg, user);
        }
    }
    0
}

pub fn channel_callback(internal: &tll_channel_internal_t, msg: *const tll_msg_t) -> i32 {
    if msg.is_null() {
        return 0;
    }
    let t = unsafe { (*msg).type_ } as u32;
    if t as tll_msg_type_t == TLL_MESSAGE_DATA {
        return channel_callback_data(internal, msg);
    };

    dump(internal, msg);
    for i in 0..internal.cb_size {
        unsafe {
            let ptr = internal.cb.offset(i as isize);
            let mask = std::ptr::addr_of!((*ptr).mask).read_unaligned();
            if mask & (1u32 << t) == 0 {
                continue;
            }
            let cb = std::ptr::addr_of!((*ptr).cb).read_unaligned();
            let user = std::ptr::addr_of!((*ptr).user).read_unaligned();
            cb.unwrap()(internal.self_, msg, user);
        }
    }
    0
}
