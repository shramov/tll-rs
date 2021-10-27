pub mod bind;
pub mod chrono;

pub use bind::*;
pub use chrono::*;

pub trait MsgId {
    const MSGID: i32;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ByteString<const SIZE : usize>
{
    pub data: [u8; SIZE],
}

impl<const SIZE : usize> ByteString<SIZE> {
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.as_slice())
    }
    pub unsafe fn as_str_unchecked(&self) -> &str {
        std::str::from_utf8_unchecked(self.as_slice())
    }
    pub fn as_slice(&self) -> &[u8] {
        match memchr::memchr(b'\x00', &self.data) {
            Some(l) => &self.data[..l],
            None => &self.data,
        }
    }
}

impl<const SIZE : usize> Binder for ByteString<SIZE> {}

#[repr(C)]
#[derive(Debug)]
pub struct OffsetPtr<T>
{
	offset: u32,
        comb: u32,
        phantom: std::marker::PhantomData<T>,
}

impl<T> OffsetPtr<T> {
    pub fn size(&self) -> usize { (self.comb & 0xffffff) as usize }
    pub fn entity(&self) -> usize { ((self.comb >> 24) & 0xf) as usize }
    pub unsafe fn data(&self) -> &[T] {
        let base = self as *const Self as *const u8;
        std::slice::from_raw_parts(&*((base.add(self.offset as usize)) as *const T), self.size())
    }
}

impl<T : Binder> Binder for OffsetPtr<T>
{
    const PRIMITIVE_BIND : bool = false;

    fn bind(data: &[u8]) -> Option<&Self> {
        let r = unsafe { bind_checked::<Self>(data)? };
        if data.len() < r.offset as usize + r.size() * r.entity() { return None }
        if <T as Binder>::PRIMITIVE_BIND {
            return Some(r);
        }

        for i in 0..r.size() {
            <T as Binder>::bind(&data[r.offset as usize + i * r.entity()..])?;
        }
        Some(r)
    }
}

pub trait SizeType { fn as_usize(&self) -> usize; }
impl SizeType for i8 { fn as_usize(&self) -> usize { *self as usize } }
impl SizeType for i16 { fn as_usize(&self) -> usize { *self as usize } }
impl SizeType for i32 { fn as_usize(&self) -> usize { *self as usize } }
impl SizeType for i64 { fn as_usize(&self) -> usize { *self as usize } }
impl SizeType for u8 { fn as_usize(&self) -> usize { *self as usize } }
impl SizeType for u16 { fn as_usize(&self) -> usize { *self as usize } }
impl SizeType for u32 { fn as_usize(&self) -> usize { *self as usize } }
impl SizeType for u64 { fn as_usize(&self) -> usize { *self as usize } }

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct Array<T, C : SizeType, const SIZE : usize>
{
	counter: C,
        array: [T; SIZE]
}

impl<T, C : SizeType, const SIZE : usize> Array<T, C, SIZE> {
    pub fn size(&self) -> usize { self.counter.as_usize() }
    pub fn data(&self) -> &[T] {
        &self.array[..self.size()]
    }
}

impl<T : Binder, C : Binder + SizeType, const SIZE : usize> Binder for Array<T, C, SIZE>
{
    const PRIMITIVE_BIND : bool = T::PRIMITIVE_BIND && C::PRIMITIVE_BIND;

    fn bind(data: &[u8]) -> Option<&Self> {
        let r = unsafe { bind_checked::<Self>(data)? };
        if <T as Binder>::PRIMITIVE_BIND {
            return Some(r);
        }

        let off = std::mem::size_of::<C>();
        for i in 0..SIZE {
            <T as Binder>::bind(&data[off + i * std::mem::size_of::<T>()..])?;
        }
        Some(r)
    }
}
