use crate::scheme::bind::*;
use num_traits::{Num, CheckedMul};
use ::chrono::{Utc, Local, TimeZone};
use std::convert::TryFrom;

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

pub trait Integer: Num + CheckedMul + std::convert::TryFrom<u64> + std::fmt::Display {}
impl Integer for i8 {}
impl Integer for u8 {}
impl Integer for i16 {}
impl Integer for u16 {}
impl Integer for i32 {}
impl Integer for u32 {}
impl Integer for i64 {}
impl Integer for u64 {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Overflow,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Duration<T, P>
where
    P: Ratio, //, T : Clone + Copy
{
    pub value: T,
    _ratio: std::marker::PhantomData<P>,
}

fn convert<T>(value: T, mul: u64, div: u64) -> Result<T, Error>
where
    T : Integer,
{
    println!("Convert {} * {} / {}", value, mul, div);
    if mul > div {
        let v = T::try_from(mul / div).map_err(|_| Error::Overflow)?;
        value.checked_mul(&v).ok_or(Error::Overflow)
    } else {
        let v = T::try_from(div / mul).map_err(|_| Error::Overflow)?;
        Ok(value / v)
    }
}

impl<T, P> Duration<T, P>
where
    P : Ratio,
    T : Integer,
{
    pub fn new(value: T) -> Self
    {
        Self { value: value, _ratio: std::marker::PhantomData }
    }

    pub fn from_duration<T1, P1>(value: Duration<T1, P1>) -> Result<Self, Error>
    where
        P1: Ratio,
        T1: Integer,
        T: std::convert::TryFrom<T1>
    {
        let value = if std::mem::size_of::<T>() > std::mem::size_of::<T1>() {
            let v0 = T::try_from(value.value).map_err(|_| Error::Overflow)?;
            let v1 = convert(v0, P1::num(), P::num())?;
            convert(v1, P::denom(), P1::denom())?
        } else {
            let v0 = convert(value.value, P1::num(), P::num())?;
            let v1 = convert(v0, P::denom(), P1::denom())?;
            T::try_from(v1).map_err(|_| Error::Overflow)?
        };
        Ok(Self::new(value))
    }
}

impl<T, P> TryFrom<Duration<T, P>> for std::time::Duration
where
    T : Integer,
    P : Ratio,
    u64: TryFrom<T>,
{
    type Error = Error;
    fn try_from(value: Duration<T, P>) -> Result<Self, Self::Error> {
        let v = Duration::<u64, Nano>::from_duration(value)?;
        Ok(std::time::Duration::from_nanos(v.value))
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

impl<T, P> TryFrom<TimePoint<T, P>> for ::chrono::DateTime<Utc>
where
    P : Ratio,
    T : Integer,
    i64 : TryFrom<T>,
{
    type Error = Error;
    fn try_from(value: TimePoint<T, P>) -> Result<Self, Self::Error> {
        value.as_datetime()
    }
}

impl<T, P> TryFrom<TimePoint<T, P>> for ::chrono::DateTime<Local>
where
    P : Ratio,
    T : Integer,
    i64 : TryFrom<T>,
{
    type Error = Error;
    fn try_from(value: TimePoint<T, P>) -> Result<Self, Self::Error> {
        value.as_local_datetime()
    }
}

impl<T, P> TryFrom<::chrono::DateTime<Utc>> for TimePoint<T, P>
where
    P : Ratio,
    T : Integer,
    T : TryFrom<i64>,
{
    type Error = Error;
    fn try_from(dt: ::chrono::DateTime<Utc>) -> Result<Self, Self::Error> {
        let d = Duration::<i64, Nano>::new(dt.timestamp_nanos());
        let v = Duration::<T, P>::from_duration(d)?;
        Ok(TimePoint { value: v })
    }
}

impl<T, P> TimePoint<T, P>
where
    P: Ratio,
    T: Integer,
    i64: TryFrom<T>,
{
    pub fn as_datetime(self) -> Result<::chrono::DateTime<Utc>, Error>
    {
        let v = Duration::<i64, Nano>::from_duration(self.value)?;
        Ok(Utc.timestamp_nanos(v.value))
    }

    pub fn as_local_datetime(self) -> Result<::chrono::DateTime<Local>, Error>
    {
        Ok(self.as_datetime()?.into())

    }
}

impl<T, P> Binder for Duration<T, P> where P: Ratio {}
impl<T, P> Binder for TimePoint<T, P> where P: Ratio {}
