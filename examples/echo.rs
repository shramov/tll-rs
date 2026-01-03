use tll::channel::*;

use tll::channel::base::*;
use tll::config::Config;
use tll::error::*;

#[derive(Debug, Default)]
struct Echo {
    base: Base,
}

impl Extension for Echo {
    type Inner = Base;

    fn inner(&self) -> &Self::Inner {
        &self.base
    }
    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.base
    }
}

impl ChannelImpl for Echo {
    fn channel_protocol() -> &'static str {
        "rs-echo"
    }

    fn open_policy() -> OpenPolicy {
        OpenPolicy::Manual
    }

    fn init(&mut self, url: &Config, master: Option<Channel>, context: &Context) -> Result<()> {
        self.logger().info(&format!("Create channel, master {:?}", master));
        self.inner_mut().init(url, master, context)
    }

    fn open(&mut self, url: &Config) -> Result<()> {
        self.logger().info("Begin channel open");
        self.inner_mut().open(url)
    }

    fn process(&mut self) -> Result<i32> {
        self.logger().info("Called process, open channel");
        if self.state() == State::Opening {
            self.set_state(State::Active);
        }
        Ok(0)
    }

    fn post(&mut self, msg: &Message) -> Result<()> {
        self.base.callback_data(msg);
        Ok(())
    }
}

tll::declare_channel_impl!(Echo);
tll::declare_channel_module!(Echo);
