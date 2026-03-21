use crate::bind::bind::BindError;
use crate::mem::{MemOffset, MemRead};

#[derive(Debug, Eq, PartialEq)]
pub enum UnionBindError {
    BindError(BindError),
    UnionError(usize),
}

impl std::fmt::Display for UnionBindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BindError(e) => write!(f, "{}", e),
            Self::UnionError(e) => write!(f, "invalid index: {e}"),
        }
    }
}

impl std::error::Error for UnionBindError {}

impl From<UnionBindError> for crate::result::Error {
    fn from(e: UnionBindError) -> Self {
        Self::from(format!("Failed to bind union: {}", e))
    }
}

impl From<BindError> for UnionBindError {
    fn from(e: BindError) -> Self {
        Self::BindError(e)
    }
}

pub trait UnionType<Buf: MemRead>: Sized {
    fn bind_index(index: usize, data: MemOffset<Buf>) -> Result<Self, UnionBindError>;
}

#[inline(always)]
pub fn union_bind<Inner: UnionType<Buf> + Sized, Index: Into<usize> + Copy, Buf: MemRead>(
    data: MemOffset<Buf>,
    offset: usize,
) -> Result<Inner, UnionBindError> {
    let index = data.mem_get_primitive::<Index>(offset).into();
    Inner::bind_index(index, data.view(offset + std::mem::size_of::<Index>()))
}
