use crate::channel::*;

pub fn channel_callback_data( internal : &tll_channel_internal_t, msg : *const tll_msg_t ) -> i32
{
        let list = unsafe { std::slice::from_raw_parts(internal.data_cb as * const tll_channel_callback_pair_t, internal.data_cb_size as usize) };
        for cb in list {
		unsafe { cb.cb.unwrap()(internal.self_, msg, cb.user) };
        }
	0
}

pub fn channel_callback( internal : &tll_channel_internal_t, msg : *const tll_msg_t ) -> i32
{
        if msg.is_null() { return 0 }
        let t = unsafe { (*msg).type_ } as u32;
        if t as tll_msg_type_t == TLL_MESSAGE_DATA { return channel_callback_data(internal, msg) };

        let list = unsafe { std::slice::from_raw_parts(internal.cb as * const tll_channel_callback_pair_t, internal.cb_size as usize) };
        for cb in list {
            if cb.mask & (1u32 << t) != 0 {
                unsafe { cb.cb.unwrap()(internal.self_, msg, cb.user) };
            }
        }
	0
}
