use tll::channel::*;

use tll::channel::impl_::*;
use tll::config::Config;
use tll::error::*;
use tll::props::Props;

use tll_sys::channel::{tll_channel_module_t, tll_channel_context_t};

#[derive(Debug, Default)]
struct Echo {
    internal: Internal,
}

impl ChannelImpl for Echo {
    fn open_policy() -> OpenPolicy {
        OpenPolicy::Manual
    }

    fn new() -> Self { Self::default() }

    fn internal(&self) -> &Internal { &self.internal }
    fn internal_mut(&mut self) -> &mut Internal { &mut self.internal }

    fn init(&mut self, _url: &Config, parent: Option<Channel>, _: &Context) -> Result<()> {
        println!("Create channel, parent {:?}", parent);
        Ok(())
    }

    fn open(&mut self, url: &Props) -> Result<()> {
        println!("Open channel {:?}", url);
        Ok(())
    }

    fn process(&mut self) -> Result<i32> {
        println!("Called process");
        if self.state() == State::Opening {
            self.set_state(State::Active);
        }
        Ok(0)
    }

    fn post(&mut self, msg: &Message) -> Result<()> {
        self.internal.callback_data(msg);
        Ok(())
    }
}

tll::declare_channel_impl!(echo_impl, Echo, "echo");
tll::declare_channel_module!(echo_impl);
