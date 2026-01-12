use crate::channel::*;
//use crate::channel::base::*;
use crate::config::Config;

#[derive(Default)]
pub struct Logic {
    pub tag: String,
    pub channels: Vec<Channel>,
    pub callback_drop: Vec<CallbackDrop>,
}

#[derive(Debug, Clone)]
pub enum LogicError {
    MissingChannel(String, String),
    Overflow(String, usize, usize),
    Underflow(String, usize, usize),
}

impl std::fmt::Display for LogicError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LogicError::MissingChannel(tag, name) => write!(f, "channel not found: '{}' for tag '{}", name, tag),
            LogicError::Overflow(tag, need, found) => {
                write!(f, "too many channels: {}, maximum {} for tag '{}'", found, need, tag)
            }
            LogicError::Underflow(tag, need, found) => write!(
                f,
                "not enought channels: {}, need at least {} for tag '{}'",
                found, need, tag
            ),
        }
    }
}

impl std::error::Error for LogicError {}

impl From<LogicError> for crate::Error {
    fn from(e: LogicError) -> Self {
        crate::Error::from(format!("{e}"))
    }
}

impl std::ops::Index<usize> for Logic {
    type Output = Channel;

    fn index(&self, index: usize) -> &Self::Output {
        self.channels.index(index)
    }
}

impl<'a> std::iter::IntoIterator for &'a Logic {
    type Item = <&'a Vec<Channel> as std::iter::IntoIterator>::Item;
    type IntoIter = <&'a Vec<Channel> as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.channels).into_iter()
    }
}

impl Logic {
    pub fn new(context: &Context, cfg: &Config, tag: &str) -> Result<Self, LogicError> {
        let channels = cfg
            .get(format!("tll.channel.{tag}"))
            .unwrap_or("".to_owned())
            .split(",")
            .map(|x| x.trim())
            .map(|x| context.get(x).ok_or(LogicError::MissingChannel(tag.to_owned(), x.to_owned())))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Logic {
            tag: tag.to_owned(),
            channels,
            callback_drop: vec![],
        })
    }

    pub fn len(&self) -> usize {
        self.channels.len()
    }

    pub fn at_least(self, min: usize) -> Result<Self, LogicError> {
        if self.channels.len() < min {
            Err(LogicError::Underflow(self.tag, min, self.channels.len()))
        } else {
            Ok(self)
        }
    }

    pub fn at_most(self, max: usize) -> Result<Self, LogicError> {
        if self.channels.len() > max {
            Err(LogicError::Overflow(self.tag, max, self.channels.len()))
        } else {
            Ok(self)
        }
    }

    pub fn with_callback<F: Callback<T>, T>(mut self, f: &F, mask: MsgMask) -> Result<Self, crate::Error> {
        self.callback_drop =
            self.channels.iter_mut().map(|c| c.callback_add::<F, T>(f, mask)).collect::<Result<Vec<_>, _>>()?;
        Ok(self)
    }

    pub fn with_callback_mut<F: CallbackMut<T>, T>(mut self, f: &mut F, mask: MsgMask) -> Result<Self, crate::Error> {
        self.callback_drop = self
            .channels
            .iter_mut()
            .map(|c| c.callback_add_mut::<F, T>(f, mask))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(self)
    }

    pub fn clear(&mut self) {
        self.callback_drop.clear();
        self.channels.clear();
    }
}
