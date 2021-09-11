pub trait ByteString
{
    fn as_str(&self) -> Result<&str, std::str::Utf8Error> { std::str::from_utf8(self.as_slice()) }
    unsafe fn as_str_unchecked(&self) -> &str { std::str::from_utf8_unchecked(self.as_slice()) }
    fn as_slice(&self) -> &[u8]
    {
        let bytes = self.get_data();
        match memchr::memchr(b'\x00', bytes) {
            Some(l) => &bytes[..l],
            None => bytes
        }
    }
    fn get_data(&self) -> &[u8];
}

pub trait MsgId
{
    const MSGID : i32;
}
