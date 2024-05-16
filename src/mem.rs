pub trait MemRead {
    fn as_mem(&self) -> &[u8];

    #[inline(always)]
    fn mem_size(&self) -> usize {
        self.as_mem().len()
    }

    #[inline(always)]
    fn mem_get_primitive<R: Copy>(&self, offset: usize) -> R {
        let ptr = self.as_mem()[offset..].as_ptr();
        unsafe { (ptr as *const R).read_unaligned() }
    }

    #[inline(always)]
    fn mem_get_bytes(&self, offset: usize, size: usize) -> &[u8] {
        &self.as_mem()[offset..offset + size]
    }

    #[inline(always)]
    fn mem_get_bytestring<'a>(&self, offset: usize, size: usize) -> &[u8] {
        let slice = &self.as_mem()[offset..offset + size];
        match memchr::memchr(b'\x00', &slice) {
            Some(l) => &slice[..l],
            None => &slice,
        }
    }
}

pub trait MemWrite: MemRead {
    fn as_mem_mut(&mut self) -> &mut [u8];

    #[inline(always)]
    fn mem_set_primitive<R: Copy>(&mut self, offset: usize, value: R) {
        let ptr = self.as_mem_mut()[offset..].as_ptr();
        unsafe { (ptr as *mut R).write_unaligned(value) }
    }

    #[inline(always)]
    fn mem_set_bytes(&mut self, offset: usize, size: usize, value: &[u8]) {
        let slice = &mut self.as_mem_mut()[offset..size];
        if value.len() < size {
            slice[..value.len()].clone_from_slice(value);
            slice[value.len()..].fill(0);
        } else {
            slice.clone_from_slice(&value[..size]);
        }
    }
}

pub trait MemGrow: MemWrite {
    fn mem_resize(&mut self, size: usize);
}

impl<T: MemRead> MemRead for &T {
    #[inline(always)]
    fn as_mem(&self) -> &[u8] {
        <T as MemRead>::as_mem(self)
    }
}

impl<T: MemRead> MemRead for &mut T {
    #[inline(always)]
    fn as_mem(&self) -> &[u8] {
        <T as MemRead>::as_mem(self)
    }
}

impl<T: MemWrite> MemWrite for &mut T {
    #[inline(always)]
    fn as_mem_mut(&mut self) -> &mut [u8] {
        <T as MemWrite>::as_mem_mut(self)
    }
}

impl<T: MemGrow> MemGrow for &mut T {
    #[inline(always)]
    fn mem_resize(&mut self, size: usize) {
        <T as MemGrow>::mem_resize(self, size)
    }
}

impl<const SIZE: usize> MemRead for [u8; SIZE] {
    #[inline(always)]
    fn as_mem(&self) -> &[u8] {
        self
    }
}

impl<const SIZE: usize> MemWrite for [u8; SIZE] {
    #[inline(always)]
    fn as_mem_mut(&mut self) -> &mut [u8] {
        self
    }
}

impl MemRead for &[u8] {
    #[inline(always)]
    fn as_mem(&self) -> &[u8] {
        self
    }
}

impl MemRead for &mut [u8] {
    #[inline(always)]
    fn as_mem(&self) -> &[u8] {
        self
    }
}

impl MemWrite for &mut [u8] {
    #[inline(always)]
    fn as_mem_mut(&mut self) -> &mut [u8] {
        self
    }
}

impl MemRead for Vec<u8> {
    #[inline(always)]
    fn as_mem(&self) -> &[u8] {
        self
    }
}

impl MemWrite for Vec<u8> {
    #[inline(always)]
    fn as_mem_mut(&mut self) -> &mut [u8] {
        self
    }
}

impl MemGrow for Vec<u8> {
    #[inline(always)]
    fn mem_resize(&mut self, size: usize) {
        self.resize(size, 0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MemOffset<T>
where
    T: MemRead,
{
    pub buf: T,
    pub offset: usize,
}

impl<Buf: MemRead> From<Buf> for MemOffset<Buf> {
    fn from(buf: Buf) -> Self {
        Self::new(buf)
    }
}

impl<T> MemOffset<T>
where
    T: MemRead,
{
    #[inline(always)]
    pub fn new(buf: T) -> Self {
        Self { buf, offset: 0 }
    }

    #[inline(always)]
    pub fn view(self, offset: usize) -> Self {
        Self {
            buf: self.buf,
            offset: offset + self.offset,
        }
    }

    #[inline(always)]
    pub fn reborrow(&mut self) -> MemOffset<&mut T> {
        MemOffset {
            buf: &mut self.buf,
            offset: self.offset,
        }
    }
}

impl<T> MemRead for MemOffset<T>
where
    T: MemRead,
{
    #[inline(always)]
    fn as_mem(&self) -> &[u8] {
        &self.buf.as_mem()[self.offset..]
    }
}

impl<T> MemWrite for MemOffset<T>
where
    T: MemWrite,
{
    #[inline(always)]
    fn as_mem_mut(&mut self) -> &mut [u8] {
        &mut self.buf.as_mem_mut()[self.offset..]
    }
}

impl<T> MemGrow for MemOffset<T>
where
    T: MemGrow,
{
    #[inline(always)]
    fn mem_resize(&mut self, size: usize) {
        self.buf.mem_resize(size + self.offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn read<T: MemRead>(buf: T, offset: usize) -> u8 {
        buf.as_mem()[offset]
    }

    fn write<T: MemWrite>(mut buf: T, offset: usize, value: u8) {
        buf.as_mem_mut()[offset] = value
    }

    #[test]
    fn test() {
        {
            let mut buf = [0u8; 16];
            write(&mut buf, 0, 0xfe);
            assert_eq!(buf[0], 0xfe);
            assert_eq!(read(&buf, 0), 0xfe);
            buf.mem_set_primitive(4, 0xdeadbeefu32);
            assert_eq!(0xdeadbeefu32, buf.mem_get_primitive::<u32>(4));
        }

        {
            let mut buf = vec![0u8; 4];
            write(&mut buf, 0, 0xfe);
            assert_eq!(buf[0], 0xfe);
            assert_eq!(read(&buf, 0), 0xfe);
            buf.mem_resize(8);
            assert_eq!(buf.len(), 8);
            assert_eq!(read(&buf, 0), 0xfe);
            buf.mem_set_primitive(4, 0xdeadbeefu32);
            assert_eq!(0xdeadbeefu32, buf.mem_get_primitive::<u32>(4));

            buf.mem_resize(16);
        }

        {
            let mut data = vec![0u8; 4];
            let mut buf = MemOffset::new(&mut data);
            write(&mut buf, 0, 0xfe);
            assert_eq!(read(&buf, 0), 0xfe);
            assert_eq!(buf.mem_get_primitive::<u8>(0), 0xfe);
            buf.mem_resize(8);
            assert_eq!(buf.mem_size(), 8);
            assert_eq!(read(&buf, 0), 0xfe);
            buf.mem_set_primitive(4, 0xdeadbeefu32);
            assert_eq!(0xdeadbeefu32, buf.mem_get_primitive::<u32>(4));

            buf.mem_resize(16);
        }
    }
}
