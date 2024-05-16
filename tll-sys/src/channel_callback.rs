use crate::channel::*;

pub fn channel_callback_data(internal: &tll_channel_internal_t, msg: *const tll_msg_t) -> i32 {
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

    for i in 0..internal.data_cb_size {
        unsafe {
            let ptr = internal.data_cb.offset(i as isize);
            let mask = std::ptr::addr_of!((*ptr).mask).read_unaligned();
            if mask & (1u32 << t) != 0 {
                continue;
            }
            let cb = std::ptr::addr_of!((*ptr).cb).read_unaligned();
            let user = std::ptr::addr_of!((*ptr).user).read_unaligned();
            cb.unwrap()(internal.self_, msg, user);
        }
    }
    0
}
