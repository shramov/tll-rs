use tll::channel::*;

use tll::channel::impl_::*;
use tll::config::Config;
use tll::error::*;
use tll::props::Props;
use tll_sys::channel::tll_state_t;

#[derive(Debug)]
struct TestPrefix {
    internal: Internal,
    child: Option<OwnedChannel>,
}

impl Default for TestPrefix
{
    fn default() -> Self
    {
        Self {
            internal: Internal::default(),
            child: None,
        }
    }
}

impl Drop for TestPrefix
{
    fn drop(&mut self)
    {
        self.child = None;
    }
}

impl CallbackMut for TestPrefix {
    fn message_callback_mut(&mut self, c : &Channel, m: &Message) -> i32
    {
        self.child_cb(c, m)
    }
}

impl ChannelImpl for TestPrefix {
    fn open_policy() -> OpenPolicy {
        OpenPolicy::Manual
    }

    fn process_policy() -> ProcessPolicy {
        ProcessPolicy::Never
    }

    fn child_policy() -> ChildPolicy {
        ChildPolicy::Single
    }

    fn internal(&self) -> &Internal {
        &self.internal
    }
    fn internal_mut(&mut self) -> &mut Internal {
        &mut self.internal
    }

    fn init(&mut self, url: &Config, _parent: Option<Channel>, ctx: &Context) -> Result<()> {
        let curl = url.copy();
        curl.set("internal", "yes");
        match url.get("tll.proto") {
            Some(proto) => {
                let v: Vec<&str> = proto.splitn(2, '+').collect();
                if v.len() == 1 {
                    self.logger().error(&format!(
                        "Invalid url: proto without '+' separator tll.proto field: '{}'",
                        proto
                    ));
                    return Err(Error::from(EINVAL));
                }
                self.logger().info(&format!("Child protocol: {}", v[1]));
                curl.set("tll.proto", v[1]);
                curl.set("name", &format!("{}/{}", v[0], self.internal().name()));
            }
            None => {
                self.logger().error("Invalid url: missing tll.proto field");
                return Err(Error::from(EINVAL));
            }
        }
        match ctx.channel_url(&curl) {
            Err(e) => {
                self.logger().error("Failed to create child channel");
                return Err(e);
            }
            Ok(mut c) => {
                c.callback_add_mut(self, None)?;
                self.child = Some(c)
            }
        }
        Ok(())
    }

    fn free(&mut self)
    {
        self.child = None;
    }

    fn open(&mut self, url: &Props) -> Result<()> {
        if let Some(c) = &mut self.child {
            c.open(&url.as_string())
        } else {
            Err(Error::from(EINVAL))
        }
    }

    fn post(&mut self, msg: &Message) -> Result<()> {
        if let Some(c) = &mut self.child {
            c.post(msg)
        } else {
            Err(Error::from(EINVAL))
        }
    }
}

impl TestPrefix {
    pub fn on_state(&mut self, s: State) -> i32 {
	match s {
            State::Active => self.on_active(),
            State::Error => self.on_error(),
            State::Closing => self.on_closing(),
            State::Closed => self.on_closed(),
	    _ => 0
	}
    }

    pub fn on_active(&mut self) -> i32 {
        self.internal_mut().set_state(State::Active);
        0
    }

    pub fn on_error(&mut self) -> i32 {
        self.internal_mut().set_state(State::Error);
        0
    }

    pub fn on_closing(&mut self) -> i32 {
        0
    }

    pub fn on_closed(&mut self) -> i32 {
        self.internal_mut().set_state(State::Closed);
        0
    }

    pub fn on_data(&mut self, msg: &Message) -> i32 {
        self.internal().callback_data(msg);
        0
    }

    pub fn on_control(&mut self, msg: &Message) -> i32 {
        self.internal().callback(msg);
        0
    }

    pub fn on_other(&mut self, msg: &Message) -> i32 {
        self.internal().callback(msg);
        0
    }

    pub fn child_cb(&mut self, _c: &Channel, msg: &Message) -> i32 {
        self.logger().info(&format!("Got message {:?}", msg));
        match msg.get_type() {
            MsgType::State => self.on_state(State::from(msg.msgid() as tll_state_t)),
            MsgType::Data => self.on_data(msg),
            MsgType::Control => self.on_control(msg),
            _ => self.on_other(msg),
        }
    }
}

tll::declare_channel_impl!(test_prefix_impl, TestPrefix, "prefix+");
tll::declare_channel_module!(test_prefix_impl);

#[test]
fn test() -> Result<()> {
    let ctx = Context::new();
    assert!(ctx.channel("prefix+null://;name=prefix").is_err());
    ctx.register(test_prefix_impl())?;

    {
        let mut r = ctx.channel("prefix+null://host;name=prefix");
        assert!(r.is_ok());
        println!("Created channel");
        let c = r.as_mut()?.get_mut();

        //assert!(c.callback_add(&callback, None).is_ok());

        assert_eq!(c.name(), "prefix");
        assert_eq!(c.state(), State::Closed);

        assert!(c.open("").is_ok());
        assert_eq!(c.state(), State::Active);

        assert!(c.post(Message::new().set_msgid(100).set_seq(100).set_data(b"abcd")).is_ok())
    }

    Ok(())
}
