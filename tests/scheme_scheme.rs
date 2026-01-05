#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

pub use tll::mem::{MemOffset, MemRead};
pub use tll::scheme::mem::ByteString;
pub use tll::scheme::*;

pub const SCHEME_STRING : &str = "yamls+gz://eJylks1qhDAQgO/7FLkFisJqZZG9dR+jpYhusktAE8nPQRbfvTO22sRYaekpmcyXbzJJUiLrjp8JpQdCuHSdOcOEEMpLeiYPO/SQdELaMpnSsEZfIJMlhF5gzGF85VrB9DiOh3TWGdeg8SZ4y76UKXnMyZIm5FNNUU29jZ2540bBoMbxR4OIDSHgPMBtEiI7BQ4II4mPuG1GPOeBBsJI4yNumxGnItBAGGl8xG0zN59hyjUtjxiW5d7lMH4VXd3i2hq8+u03g+VTrHorlDT4ETBF8T2tFvJOx7WgKcP9cYla62J1fW/Fe4T1Vn9TT9ut449bGAyivp2u8eiVM+sXC7rS3KjWYYi9TfDS6eyIe13s0kSvtKOXv9Rb0fFega5i9fCX8wPuV0BPNYn2avynh70K3PsQ02/4ABSaUBY=";

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum e8 {
    Zero = 0,
    A = 1,
    B = 2,
}
impl Binder for e8 {}

#[derive(Debug)]
pub struct sub<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> sub<Buf> {
    pub fn bind(data: Buf) -> std::result::Result<Self, BindError> {
        if data.mem_size() < 1 {
            return Err(BindError::new_size(1));
        }
        Ok(Self {
            data: MemOffset::new(data),
        })
    }

    pub fn bind_unchecked(data: Buf) -> Self {
        Self {
            data: MemOffset::new(data),
        }
    }

    pub fn get_s8(&self) -> i8 {
        self.data.mem_get_primitive::<i8>(0)
    }
}
#[derive(Debug)]
pub struct msg<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> msg<Buf> {
    pub fn bind(data: Buf) -> std::result::Result<Self, BindError> {
        if data.mem_size() < 129 {
            return Err(BindError::new_size(129));
        }
        // Array
        // Pointer
        sub::<&[u8]>::bind(&data.as_mem()[103..])?;
        Ok(Self {
            data: MemOffset::new(data),
        })
    }

    pub fn bind_unchecked(data: Buf) -> Self {
        Self {
            data: MemOffset::new(data),
        }
    }

    pub fn get_i8(&self) -> i8 {
        self.data.mem_get_primitive::<i8>(0)
    }
    pub fn get_u8(&self) -> u8 {
        self.data.mem_get_primitive::<u8>(1)
    }
    pub fn get_i16(&self) -> i16 {
        self.data.mem_get_primitive::<i16>(2)
    }
    pub fn get_u16(&self) -> u16 {
        self.data.mem_get_primitive::<u16>(4)
    }
    pub fn get_i32(&self) -> i32 {
        self.data.mem_get_primitive::<i32>(6)
    }
    pub fn get_u32(&self) -> u32 {
        self.data.mem_get_primitive::<u32>(10)
    }
    pub fn get_i64(&self) -> i64 {
        self.data.mem_get_primitive::<i64>(14)
    }
    pub fn get_u64(&self) -> u64 {
        self.data.mem_get_primitive::<u64>(22)
    }
    pub fn get_f64(&self) -> f64 {
        self.data.mem_get_primitive::<f64>(30)
    }
    pub fn get_d128(&self) -> tll::decimal128::Decimal128 {
        self.data.mem_get_primitive::<tll::decimal128::Decimal128>(38)
    }
    pub fn get_c16(&self) -> tll::scheme::mem::ByteString<16, MemOffset<Buf>> {
        tll::scheme::mem::ByteString::<16, MemOffset<Buf>>::new(self.data.view(54))
    }
    pub fn get_b8(&self) -> &[u8] {
        &self.data.as_mem()[70..70 + 8]
    }
    pub fn get_arr4(&self) -> tll::scheme::mem::Array<u8, i32, 4, MemOffset<Buf>> {
        tll::scheme::mem::Array::<u8, i32, 4, MemOffset<Buf>>::new(self.data.view(78))
    }
    pub fn get_ptr(
        &self,
    ) -> Result<tll::scheme::mem::OffsetPtr<i64, tll::scheme::mem::OffsetPtrDefault, Buf>, BindError> {
        tll::scheme::mem::OffsetPtr::<i64, tll::scheme::mem::OffsetPtrDefault, Buf>::new(self.data.view(95))
    }
    pub fn get_sub(&self) -> sub<Buf> {
        sub::<Buf> {
            data: self.data.view(103),
        }
    }
    pub fn get_duration_us(&self) -> tll::scheme::Duration<i32, tll::scheme::Micro> {
        self.data.mem_get_primitive::<tll::scheme::Duration<i32, tll::scheme::Micro>>(104)
    }
    pub fn get_duration_ns(&self) -> tll::scheme::Duration<u64, tll::scheme::Nano> {
        self.data.mem_get_primitive::<tll::scheme::Duration<u64, tll::scheme::Nano>>(108)
    }
    pub fn get_timepoint_days(&self) -> tll::scheme::TimePoint<i32, tll::scheme::RatioDay> {
        self.data.mem_get_primitive::<tll::scheme::TimePoint<i32, tll::scheme::RatioDay>>(116)
    }
    pub fn get_timepoint_ns(&self) -> tll::scheme::TimePoint<u64, tll::scheme::Nano> {
        self.data.mem_get_primitive::<tll::scheme::TimePoint<u64, tll::scheme::Nano>>(120)
    }
    pub fn get_e8(&self) -> e8 {
        self.data.mem_get_primitive::<e8>(128)
    }
}
impl<Buf: MemRead> MsgId for msg<Buf> {
    const MSGID: i32 = 10;
}
