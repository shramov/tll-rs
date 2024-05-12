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
    fn channel_protocol() -> &'static str { "echo" }

    fn open_policy() -> OpenPolicy {
        OpenPolicy::Manual
    }

    fn init(&mut self, url: &Config, master: Option<Channel>, context: &Context) -> Result<()> {
        println!("Create channel, master {:?}", master);
        self.inner_mut().init(url, master, context)
    }

    fn open(&mut self, url: &Config) -> Result<()> {
        println!("Open channel");
        self.inner_mut().open(url)
    }

    fn process(&mut self) -> Result<i32> {
        println!("Called process");
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

tll::declare_channel_impl!(echo_impl, Echo);
tll::declare_channel_module!(echo_impl);
