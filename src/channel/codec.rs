use crate::channel::channel::*;
use crate::channel::impl_::*;

use crate::config::Config;
use crate::error::*;
use tll_sys::channel::tll_state_t;

#[derive(Debug)]
pub struct Codec<T: CodecImpl> {
    base: Base,
    codec: T,
    child: OwnedChannel,
}

impl<T> Default for Codec<T>
where
    T : CodecImpl
{
    fn default() -> Self {
        Codec { base: Base::default(), codec: T::default(), child: unsafe { OwnedChannel::new_null() } }
    }
}

pub trait CodecImpl: Default {
    fn encode(&mut self, m: &Message) -> Result<Message>;
    fn decode(&mut self, m: &Message) -> Result<Message>;
}

impl<T: CodecImpl> Drop for Codec<T> {
    fn drop(&mut self) {
        self.child = unsafe { OwnedChannel::new_null() };
    }
}

impl<T: CodecImpl> Extension for Codec<T> {
    type Inner = Base;

    fn inner(&self) -> &Self::Inner {
        &self.base
    }
    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.base
    }
}

impl<T: CodecImpl> ChannelImpl for Codec<T> {
    fn open_policy() -> OpenPolicy {
        OpenPolicy::Manual
    }
    fn process_policy() -> ProcessPolicy {
        ProcessPolicy::Never
    }
    fn child_policy() -> ChildPolicy {
        ChildPolicy::Single
    }

    fn init(&mut self, url: &Config, master: Option<Channel>, ctx: &Context) -> Result<()> {
        let log = self.logger();
        let curl = url.copy();
        curl.set("internal", "yes");
        match url.get("tll.proto") {
            Some(proto) => {
                let v: Vec<&str> = proto.splitn(2, '+').collect();
                if v.len() == 1 {
                    log.error(&format!(
                        "Invalid url: proto without '+' separator tll.proto field: '{}'",
                        proto
                    ));
                    return Err(Error::from(EINVAL));
                }
                log.info(&format!("Child protocol: {}", v[1]));
                curl.set("tll.proto", v[1]);
                curl.set("name", &format!("{}/{}", v[0], self.base().name()));
            }
            None => {
                log.error("Invalid url: missing tll.proto field");
                return Err(Error::from(EINVAL));
            }
        }
        match ctx.channel_url(&curl) {
            Err(e) => {
                log.error("Failed to create child channel");
                return Err(e);
            }
            Ok(mut c) => {
                c.callback_add_mut(self, None)?;
                self.child = c;
            }
        }
        self.inner_mut().init(url, master, ctx)
    }

    fn free(&mut self) {}

    fn open(&mut self, cfg: &Config) -> Result<()> {
        self.inner_mut().open(cfg)?;
        self.child.open_cfg(cfg)
    }

    fn close(&mut self, force: bool) {
        self.child.close_force(force)
    }

    fn post(&mut self, msg: &Message) -> Result<()> {
        let m = self.codec.encode(msg)?;
        self.child.post(&m)
    }

    fn process(&mut self) -> Result<i32> {
        Ok(EAGAIN)
    }
}

impl<T: CodecImpl> Codec<T> {
    fn on_state(&mut self, s: State) -> Result<()> {
        match s {
            State::Active => self.on_active(),
            State::Error => self.on_error(),
            State::Closing => self.on_closing(),
            State::Closed => self.on_closed(),
            _ => Ok(()),
        }
    }

    fn on_active(&mut self) -> Result<()> {
        self.set_state(State::Active);
        Ok(())
    }

    fn on_error(&mut self) -> Result<()> {
        self.set_state(State::Error);
        Ok(())
    }

    fn on_closing(&mut self) -> Result<()> {
        Ok(())
    }

    fn on_closed(&mut self) -> Result<()> {
        self.set_state(State::Closed);
        Ok(())
    }

    fn on_data(&mut self, msg: &Message) -> Result<()> {
        let m = self.codec.decode(msg)?;
        self.base().callback_data(&m);
        Ok(())
    }

    fn on_control(&mut self, msg: &Message) -> Result<()> {
        self.base().callback(msg);
        Ok(())
    }

    fn on_other(&mut self, msg: &Message) -> Result<()> {
        self.base().callback(msg);
        Ok(())
    }

    fn on_message(&mut self, _c: &Channel, msg: &Message) -> i32 {
        self.logger().info(&format!("Got message {:?}", msg));
        let r = match msg.get_type() {
            MsgType::State => self.on_state(State::from(msg.msgid() as tll_state_t)),
            MsgType::Data => self.on_data(msg),
            MsgType::Control => self.on_control(msg),
            _ => self.on_other(msg),
        };
        if let Err(e) = r {
            self.logger().error(&format!("Failed to handle message {:?}: {}", msg, e));
            if self.state() != State::Closed { self.set_state(State::Error); }
            return EINVAL;
        }
        0
    }
}

impl<T: CodecImpl> CallbackMut for Codec<T> {
    fn message_callback_mut(&mut self, c: &Channel, msg: &Message) -> i32 {
        self.on_message(c, msg)
    }
}
