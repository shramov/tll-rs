use crate::decimal128::Decimal128;

#[derive(Debug,Eq,PartialEq)]
pub struct BindError {
}

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

#[derive(Debug,Eq,PartialEq)]
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

pub unsafe fn bind_unchecked<T>(data: &[u8]) -> &T {
    &*(data.as_ptr() as *const T)
}

pub unsafe fn bind_checked<T>(data: &[u8]) -> Option<&T> {
    if data.len() < std::mem::size_of::<T>() {
        return None;
    }
    Some(bind_unchecked::<T>(data))
}

pub trait Binder: Sized {
    const PRIMITIVE_BIND: bool = true;
    fn bind(data: &[u8]) -> Option<&Self> {
        unsafe { bind_checked::<Self>(data) }
    }
}

impl Binder for i8 {}
impl Binder for u8 {}
impl Binder for i16 {}
impl Binder for u16 {}
impl Binder for i32 {}
impl Binder for u32 {}
impl Binder for i64 {}
impl Binder for u64 {}
impl Binder for f64 {}
impl Binder for Decimal128 {}

impl<T, const SIZE: usize> Binder for [T; SIZE]
where
    T: Binder,
{
    const PRIMITIVE_BIND: bool = T::PRIMITIVE_BIND;

    fn bind(data: &[u8]) -> Option<&Self> {
        if <T as Binder>::PRIMITIVE_BIND {
            unsafe { bind_checked::<Self>(data) }
        } else {
            let r = unsafe { bind_checked::<Self>(data) }?;
            for i in 0..SIZE {
                <T as Binder>::bind(&data[i * std::mem::size_of::<T>()..])?;
            }
            Some(r)
        }
    }
}

pub fn bind_check_inner<T: Binder>(data: &[u8]) -> Option<()> {
    if !<T as Binder>::PRIMITIVE_BIND {
        <T as Binder>::bind(data)?;
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench() {
        let data = [0; 1024];
        let mut start = std::time::SystemTime::now();
        for _ in 0..1000000 {
            <u8 as Binder>::bind(&data);
        }
        let mut dt = start.elapsed().unwrap();
        println!("Time (0): {:?}", dt);

        start = std::time::SystemTime::now();
        for _ in 0..1000000 {
            <[u8; 256] as Binder>::bind(&data);
        }
        dt = start.elapsed().unwrap();

        println!("Time (256): {:?}", dt);
        start = std::time::SystemTime::now();
        for _ in 0..1000000 {
            <[u8; 512] as Binder>::bind(&data);
        }
        dt = start.elapsed().unwrap();
        println!("Time (512): {:?}", dt);
        //assert!(false);
    }
}
