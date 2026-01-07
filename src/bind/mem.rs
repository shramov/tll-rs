use crate::bind::bind::{BindError, Binder, StringBindError};
use crate::mem::{MemOffset, MemRead};

pub struct ByteString<const SIZE: usize, Buf> {
    pub data: Buf,
}

impl<const SIZE: usize, Buf> ByteString<SIZE, Buf> {
    pub fn new(data: Buf) -> Self {
        Self { data }
    }
}

impl<const SIZE: usize, Buf: MemRead> ByteString<SIZE, Buf> {
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.as_slice())
    }
    pub unsafe fn as_str_unchecked(&self) -> &str {
        std::str::from_utf8_unchecked(self.as_slice())
    }
    pub fn as_slice(&self) -> &[u8] {
        self.data.mem_get_bytestring(0, SIZE)
    }
}

#[inline(always)]
pub fn byte_str<Buf: MemRead>(data: &MemOffset<Buf>, offset: usize, size: usize) -> Result<&'_ str, StringBindError> {
    str::from_utf8(data.mem_get_bytestring(offset, size)).map_err(StringBindError::from)
}

#[derive(Debug)]
pub struct ArrayView<Inner, Buf: MemRead> {
    data: MemOffset<Buf>,
    size: usize,
    entity: usize,
    phantom: std::marker::PhantomData<Inner>,
}

impl<Inner, Buf: MemRead> ArrayView<Inner, Buf> {
    pub fn new(data: MemOffset<Buf>, size: usize, entity: usize) -> Self {
        ArrayView {
            data,
            size,
            entity,
            phantom: std::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub fn iter(self) -> ArrayIter<Inner, Buf> {
        ArrayIter { array: self, index: 0 }
    }
}

impl<Inner: Binder<Buf>, Buf: MemRead + Copy> ArrayView<Inner, Buf> {
    #[inline(always)]
    pub fn get(&self, idx: usize) -> Option<Inner> {
        if idx > self.size {
            return None;
        }
        Some(self.get_unchecked(idx))
    }

    #[inline(always)]
    pub fn get_unchecked(&self, idx: usize) -> Inner {
        Inner::bind_unchecked(self.data.view(idx * self.entity))
    }
}

pub struct ArrayIter<Inner, Buf: MemRead> {
    array: ArrayView<Inner, Buf>,
    index: usize,
}

impl<Inner: Binder<Buf>, Buf: MemRead + Copy> IntoIterator for ArrayView<Inner, Buf> {
    type Item = Inner;
    type IntoIter = ArrayIter<Inner, Buf>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        ArrayIter { array: self, index: 0 }
    }
}

impl<Inner: Binder<Buf>, Buf: MemRead + Copy> std::iter::Iterator for ArrayIter<Inner, Buf> {
    type Item = Inner;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.size {
            let idx = self.index;
            self.index += 1;
            Some(self.array.get_unchecked(idx))
        } else {
            None
        }
    }
}

pub struct Array<Counter, Inner, const SIZE: usize, Buf: MemRead> {
    buf: MemOffset<Buf>,
    phantom: std::marker::PhantomData<(Counter, Inner)>,
}

impl<Counter, Inner, const SIZE: usize, Buf: MemRead> Binder<Buf> for Array<Counter, Inner, SIZE, Buf> {
    #[inline(always)]
    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        Self {
            buf: data,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<Counter, Inner, const SIZE: usize, Buf: MemRead> Array<Counter, Inner, SIZE, Buf> {
    pub fn new(data: Buf) -> Self {
        Self::bind_unchecked(data.into())
    }
}

impl<Counter, Inner, const SIZE: usize, Buf> Array<Counter, Inner, SIZE, Buf>
where
    Buf: MemRead + Copy,
    usize: From<Counter>,
    Counter: Copy,
{
    pub fn size(&self) -> usize {
        usize::from(self.buf.mem_get_primitive::<Counter>(0))
    }

    pub fn data(&self) -> ArrayView<Inner, Buf> {
        ArrayView::<Inner, Buf>::new(
            self.buf.view(std::mem::size_of::<Counter>()),
            self.size(),
            std::mem::size_of::<Inner>(),
        )
    }

    pub fn iter(&self) -> ArrayIter<Inner, Buf> {
        ArrayIter {
            array: self.data(),
            index: 0,
        }
    }
}

impl<Counter, Inner, const SIZE: usize, Buf> IntoIterator for Array<Counter, Inner, SIZE, Buf>
where
    Inner: Binder<Buf>,
    Buf: MemRead + Copy,
    usize: From<Counter>,
    Counter: Copy,
{
    type Item = Inner;
    type IntoIter = ArrayIter<Inner, Buf>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<Counter, Inner, const SIZE: usize, Buf> Array<Counter, Inner, SIZE, Buf>
where
    Buf: MemRead + Copy,
    Inner: Binder<Buf>,
    Counter: Copy,
{
    pub fn index(&self, index: usize) -> Inner {
        Inner::bind_unchecked(self.buf.view(std::mem::size_of::<Counter>() + std::mem::size_of::<Inner>() * index))
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GenericOffsetPtr {
    pub offset: u32,
    pub size: u32,
    pub entity: u32,
}

pub trait OffsetPtrImpl {
    const SIZE: usize;
    fn offset<Buf: MemRead>(buf: &Buf) -> usize;
    fn size<Buf: MemRead>(buf: &Buf) -> usize;
    fn entity<Buf: MemRead>(buf: &Buf) -> usize;

    //fn write<Buf: MemWrite>(&self, buf: &mut Buf, ptr: &GenericOffsetPtr);
}

pub struct OffsetPtrDefault {}

impl OffsetPtrImpl for OffsetPtrDefault {
    const SIZE: usize = 8;

    fn offset<Buf: MemRead>(buf: &Buf) -> usize {
        buf.mem_get_primitive::<u32>(0) as usize
    }

    fn size<Buf: MemRead>(buf: &Buf) -> usize {
        let ptr = &buf.as_mem()[4..];
        (ptr[0] as usize) | ((ptr[1] as usize) << 8) | ((ptr[2] as usize) << 16)
    }

    fn entity<Buf: MemRead>(buf: &Buf) -> usize {
        let v = buf.mem_get_primitive::<u8>(7) as usize;
        if v == 255 {
            buf.mem_get_primitive::<u32>(Self::offset(buf)) as usize
        } else {
            v
        }
    }
}

pub struct OffsetPtrLegacyShort {}

impl OffsetPtrImpl for OffsetPtrLegacyShort {
    const SIZE: usize = 4;

    fn offset<Buf: MemRead>(buf: &Buf) -> usize {
        buf.mem_get_primitive::<u16>(0) as usize
    }

    fn size<Buf: MemRead>(buf: &Buf) -> usize {
        buf.mem_get_primitive::<u16>(2) as usize
    }

    fn entity<Buf: MemRead>(_buf: &Buf) -> usize {
        0
    }
}

pub struct OffsetPtrLegacyLong {}

impl OffsetPtrImpl for OffsetPtrLegacyLong {
    const SIZE: usize = 8;

    fn offset<Buf: MemRead>(buf: &Buf) -> usize {
        buf.mem_get_primitive::<u32>(0) as usize
    }

    fn size<Buf: MemRead>(buf: &Buf) -> usize {
        buf.mem_get_primitive::<u16>(4) as usize
    }

    fn entity<Buf: MemRead>(buf: &Buf) -> usize {
        buf.mem_get_primitive::<u16>(6) as usize
    }
}

pub struct OffsetPtr<Inner, Ptr: OffsetPtrImpl, Buf: MemRead> {
    buf: MemOffset<Buf>,
    phantom: std::marker::PhantomData<(Inner, Ptr)>,
}

impl<Inner, Ptr: OffsetPtrImpl, Buf: MemRead> Binder<Buf> for OffsetPtr<Inner, Ptr, Buf> {
    #[inline(always)]
    fn bind_view(data: MemOffset<Buf>) -> Result<Self, BindError> {
        let size = data.mem_size();
        if size < std::mem::size_of::<Ptr>() {
            return Err(BindError::new_size(std::mem::size_of::<Ptr>()));
        }
        let offset = Ptr::offset(&data);
        let full = Ptr::size(&data) * Ptr::entity(&data);
        if full != 0 && offset + full > size {
            Err(BindError::new_size(offset + full))
        } else {
            Ok(Self::bind_unchecked(data))
        }
    }

    #[inline(always)]
    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        Self{buf: data, phantom: std::marker::PhantomData}
    }
}

impl<Inner, Ptr: OffsetPtrImpl, Buf> OffsetPtr<Inner, Ptr, Buf>
where
    Buf: MemRead,
{
    #[inline(always)]
    pub fn new(data: MemOffset<Buf>) -> Result<Self, BindError> {
        Self::bind_view(data)
    }

    #[inline(always)]
    pub fn new_unchecked(data: MemOffset<Buf>) -> Self {
        Self::bind_unchecked(data)
    }

    pub fn size(&self) -> usize {
        <Ptr as OffsetPtrImpl>::size(&self.buf) as usize
    }

    pub fn offset(&self) -> usize {
        <Ptr as OffsetPtrImpl>::offset(&self.buf) as usize
    }

    pub fn entity(&self) -> usize {
        <Ptr as OffsetPtrImpl>::entity(&self.buf) as usize
    }
}

impl<Inner, Ptr: OffsetPtrImpl, Buf> OffsetPtr<Inner, Ptr, Buf>
where
    Buf: MemRead + Copy,
{
    pub fn data(&self) -> MemOffset<Buf> {
        self.buf.view(self.offset())
    }

    pub fn array(&self) -> ArrayView<Inner, Buf> {
        ArrayView::new(self.buf.view(self.offset()), self.size(), self.entity())
    }

    pub fn iter(&self) -> ArrayIter<Inner, Buf> {
        ArrayIter {
            array: self.array(),
            index: 0,
        }
    }
}

impl<Inner: Binder<Buf>, Ptr: OffsetPtrImpl, Buf: MemRead + Copy> IntoIterator for OffsetPtr<Inner, Ptr, Buf> {
    type Item = Inner;
    type IntoIter = ArrayIter<Inner, Buf>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct OffsetString<Ptr: OffsetPtrImpl, Buf: MemRead>(OffsetPtr<u8, Ptr, Buf>);

impl<Ptr: OffsetPtrImpl, Buf: MemRead> Binder<Buf> for OffsetString<Ptr, Buf> {
    #[inline(always)]
    fn bind_view(data: MemOffset<Buf>) -> Result<Self, BindError> {
        OffsetPtr::<u8, Ptr, Buf>::bind_view(data).map(|x| Self(x))
    }

    #[inline(always)]
    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        Self(OffsetPtr::<u8, Ptr, Buf>::bind_unchecked(data))
    }
}

impl<Ptr: OffsetPtrImpl, Buf> OffsetString<Ptr, Buf>
where
    Buf: MemRead,
{
    #[inline(always)]
    pub fn new(data: MemOffset<Buf>) -> Result<Self, BindError> {
        OffsetPtr::<u8, Ptr, Buf>::new(data).map(|x| Self(x))
    }

    #[inline(always)]
    pub fn new_unchecked(data: MemOffset<Buf>) -> Self {
        Self(OffsetPtr::<u8, Ptr, Buf>::new_unchecked(data))
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        std::cmp::max(self.0.size(), 1) - 1
    }

    #[inline(always)]
    pub fn data(&self) -> Result<&'_ str, std::str::Utf8Error> {
        let size = self.size();
        let offset = self.0.offset();
        if size == 0 {
            Ok("")
        } else {
            str::from_utf8(&self.0.buf.as_mem()[offset..offset + size])
        }
    }
}

#[inline(always)]
pub fn offset_str<Ptr: OffsetPtrImpl, Buf: MemRead>(
    data: &MemOffset<Buf>,
    mut offset: usize,
) -> Result<&'_ str, StringBindError> {
    let this = OffsetString::<Ptr, &[u8]>::new(MemOffset::new(data.as_mem()).view(offset))?;
    let size = this.size();
    offset += this.0.offset();
    if size == 0 {
        Ok("")
    } else {
        str::from_utf8(&data.as_mem()[offset..offset + size]).map_err(StringBindError::from)
    }
}

pub struct PMap<Buf: MemRead>(Buf);

impl<Buf: MemRead> PMap<Buf> {
    #[inline(always)]
    pub fn new(buf: Buf) -> Self {
        Self(buf)
    }

    #[inline(always)]
    pub fn get(&self, index: i32) -> bool {
        if index < 0 {
            return true;
        }
        self.0.mem_get_primitive::<u8>(index as usize / 8) & (1 << (index & 0xf)) != 0
    }
}
