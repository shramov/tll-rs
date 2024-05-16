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
}

impl<Inner: Copy, Buf: MemRead> ArrayView<Inner, Buf> {
    #[inline(always)]
    pub fn get(&self, idx: usize) -> Option<Inner> {
        if idx > self.size {
            return None;
        }
        Some(self.get_unchecked(idx))
    }

    #[inline(always)]
    pub fn get_unchecked(&self, idx: usize) -> Inner {
        self.data.mem_get_primitive::<Inner>(idx * self.entity)
    }

    #[inline(always)]
    pub fn iter(self) -> ArrayIter<Inner, Buf> {
        ArrayIter { array: self, index: 0 }
    }
}

pub struct ArrayIter<Inner, Buf: MemRead> {
    array: ArrayView<Inner, Buf>,
    index: usize,
}

impl<Inner: Copy, Buf: MemRead> IntoIterator for ArrayView<Inner, Buf> {
    type Item = Inner;
    type IntoIter = ArrayIter<Inner, Buf>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        ArrayIter { array: self, index: 0 }
    }
}

impl<Inner: Copy, Buf: MemRead> std::iter::Iterator for ArrayIter<Inner, Buf> {
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

impl<Counter, Inner, const SIZE: usize, Buf: MemRead> Array<Counter, Inner, SIZE, Buf> {
    pub fn new(data: Buf) -> Self {
        Self {
            buf: data.into(),
            phantom: std::marker::PhantomData,
        }
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
    Inner: Copy,
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
    Buf: MemRead,
    Inner: Copy,
    Counter: Copy,
{
    pub fn index(&self, index: usize) -> Inner {
        self.buf
            .mem_get_primitive::<Inner>(std::mem::size_of::<Counter>() + std::mem::size_of::<Inner>() * index)
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
        buf.mem_get_primitive::<u8>(7) as usize
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

impl<Inner, Ptr: OffsetPtrImpl, Buf> OffsetPtr<Inner, Ptr, Buf>
where
    Buf: MemRead,
{
    #[inline(always)]
    pub fn new(data: MemOffset<Buf>) -> Option<Self> {
        let size = data.mem_size();
        if size < std::mem::size_of::<Ptr>() {
            return None;
        }
        let offset = Ptr::offset(&data);
        let full = Ptr::size(&data) * Ptr::entity(&data);
        if full != 0 && offset + full > size {
            return None;
        } else {
            Some(Self::new_unchecked(data))
        }
    }

    #[inline(always)]
    pub fn new_unchecked(data: MemOffset<Buf>) -> Self {
        Self {
            buf: data,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<Inner, Ptr: OffsetPtrImpl, Buf> OffsetPtr<Inner, Ptr, Buf>
where
    Buf: MemRead + Copy,
{
    pub fn size(&self) -> usize {
        <Ptr as OffsetPtrImpl>::size(&self.buf) as usize
    }

    pub fn offset(&self) -> usize {
        <Ptr as OffsetPtrImpl>::offset(&self.buf) as usize
    }

    pub fn entity(&self) -> usize {
        <Ptr as OffsetPtrImpl>::entity(&self.buf) as usize
    }

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

impl<Inner: Copy, Ptr: OffsetPtrImpl, Buf: MemRead + Copy> IntoIterator for OffsetPtr<Inner, Ptr, Buf> {
    type Item = Inner;
    type IntoIter = ArrayIter<Inner, Buf>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
