use crate::decimal128::Decimal128;
use crate::mem::{MemOffset, MemRead};
use crate::scheme::chrono::*;

#[derive(Debug, Eq, PartialEq)]
pub struct BindError {}

impl BindError {
    pub fn new_size(_: usize) -> Self {
        BindError {}
    }
}

impl std::fmt::Display for BindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "data size to small")
    }
}

impl std::error::Error for BindError {}

impl From<BindError> for crate::result::Error {
    fn from(e: BindError) -> Self {
        Self::from(format!("Failed to bind message string: {}", e))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum StringBindError {
    BindError(BindError),
    StringError(std::str::Utf8Error),
}

impl std::fmt::Display for StringBindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BindError(_) => write!(f, "pointer out of buffer"),
            Self::StringError(e) => write!(f, "invalid string: {e}"),
        }
    }
}

impl std::error::Error for StringBindError {}

impl From<StringBindError> for crate::result::Error {
    fn from(e: StringBindError) -> Self {
        Self::from(format!("Failed to bind offset string: {}", e))
    }
}

impl From<BindError> for StringBindError {
    fn from(e: BindError) -> Self {
        Self::BindError(e)
    }
}

impl From<std::str::Utf8Error> for StringBindError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::StringError(e)
    }
}

pub trait BinderCopy: Copy {}
pub trait Binder<Buf: MemRead>: Sized {
    const PRIMITIVE_BIND: bool = true;
    #[inline(always)]
    fn bind(data: Buf) -> Result<Self, BindError> {
        Self::bind_view(data.into())
    }

    #[inline(always)]
    fn bind_view(data: MemOffset<Buf>) -> Result<Self, BindError> {
        if data.mem_size() < std::mem::size_of::<Self>() {
            return Err(BindError::new_size(std::mem::size_of::<Self>()));
        } else {
            Ok(Self::bind_unchecked(data))
        }
    }

    fn bind_unchecked(data: MemOffset<Buf>) -> Self;
}

impl<Buf: MemRead, T: BinderCopy> Binder<Buf> for T {
    #[inline(always)]
    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        data.mem_get_primitive::<Self>(0)
    }
}

impl BinderCopy for i8 {}
impl BinderCopy for u8 {}
impl BinderCopy for i16 {}
impl BinderCopy for u16 {}
impl BinderCopy for i32 {}
impl BinderCopy for u32 {}
impl BinderCopy for i64 {}
impl BinderCopy for u64 {}
impl BinderCopy for f64 {}
impl BinderCopy for Decimal128 {}

impl<T: Copy, P: Ratio> BinderCopy for Duration<T, P> {}
impl<T: Copy, P: Ratio> BinderCopy for TimePoint<T, P> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench() {
        let data = [0u8; 1024];
        let start = std::time::SystemTime::now();
        for _ in 0..1000000 {
            <u8 as Binder<&[u8]>>::bind((&data[..]).into()).unwrap();
        }
        let dt = start.elapsed().unwrap();
        println!("Time (0): {:?}", dt);
        //assert!(false);
    }
}
