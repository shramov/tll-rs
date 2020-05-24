#include <tll/channel.h>
#include <tll/channel/impl.h>

int tll_channel_callback_data_s(const tll_channel_internal_t * in, const tll_msg_t * msg) { return tll_channel_callback_data(in, msg); }
int tll_channel_callback_s(const tll_channel_internal_t * in, const tll_msg_t * msg) { return tll_channel_callback(in, msg); }
