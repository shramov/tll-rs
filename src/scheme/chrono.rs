use crate::scheme::bind::*;

pub trait Ratio {
    fn num() -> u64 { 1 }
    fn denom() -> u64 { 1 }
}

#[derive(Debug, Clone, Copy)]
pub struct Nano {}
impl Ratio for Nano {
    fn denom() -> u64 { 1000000000 }
}

#[derive(Debug, Clone, Copy)]
pub struct Micro {}
impl Ratio for Micro {
    fn denom() -> u64 { 1000000 }
}

#[derive(Debug, Clone, Copy)]
pub struct Milli {}
impl Ratio for Milli {
    fn denom() -> u64 { 1000 }
}

#[derive(Debug, Clone, Copy)]
pub struct Ratio1 {}
impl Ratio for Ratio1 {}

#[derive(Debug, Clone, Copy)]
pub struct RatioMinute {}
impl Ratio for RatioMinute {
    fn num() -> u64 { 60 }
}

#[derive(Debug, Clone, Copy)]
pub struct RatioHour {}
impl Ratio for RatioHour {
    fn num() -> u64 { 3600 }
}

#[derive(Debug, Clone, Copy)]
pub struct RatioDay {}
impl Ratio for RatioDay {
    fn num() -> u64 { 86400 }
}

/*
trait Integer { fn to_u64(&self); }
impl Integer for i8 { to_u64(&self) { u64::from(self)}
impl Integer for u8 {}
impl Integer for i16 {}
impl Integer for u16 {}
impl Integer for i32 {}
impl Integer for u32 {}
impl Integer for i64 {}
*/

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Duration<T, P>
where
    P: Ratio, //, T : Clone + Copy
{
    pub value: T,
    _ratio: std::marker::PhantomData<P>,
}

impl<T, P> Into<std::time::Duration> for Duration<T, P>
where
    T: std::convert::Into<u64>,
    P: Ratio,
{
    fn into(self) -> std::time::Duration {
        let v = Into::<u64>::into(self.value);
        std::time::Duration::from_nanos(v * P::num() * Nano::denom() / P::denom())
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TimePoint<T, P>
where
    P: Ratio,
{
    pub value: Duration<T, P>,
}

impl<T, P> Binder for Duration<T, P> where P: Ratio {}
impl<T, P> Binder for TimePoint<T, P> where P: Ratio {}
