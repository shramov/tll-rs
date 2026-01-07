#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

pub use tll::bind::*;

pub const SCHEME_STRING : &str = "yamls+gz://eJyNUU1Lw0AQvedXzG1BEmhtBcmt6g8QwXPZZsdmsftBZhMJIf+9M02MKEo97XvMm5n3dgrw2mEJSmUAISYbPJUwqCrGQioUdYWK663Bbk9VjQ7VmBWfbc9NiNikXtrfLJ4MlYwAChhmhTwqh9RHYZQa64884buo06f2N9Wy5wk7y0a4zZoS1isG6Fs3b1O7Spyzz2GaYH26zy8KCbMzhkvrHNSD9QJvGT7W2h8l2obJC7rQCblj8uoPk2x7Ie8+fMjo1Tj+GVJP+5cAs5+fMak9UE8J3dUPYdm/Po6vEnWqr+ridCeL9CW9WY43ZmeXSZzS";

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Unknown = 0,
    Add = 1,
    Bind = 2,
    Change = 3,
    Unbind = 4,
    Remove = 5,
}
impl BinderCopy for Action {}

#[derive(Debug)]
pub struct Property<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> Binder<Buf> for Property<Buf> {
    fn bind_view(data: MemOffset<Buf>) -> Result<Self, BindError> {
        if data.mem_size() < 16 {
            return Err(BindError::new_size(16));
        }
        // Pointer
        // Pointer
        Ok(Self { data })
    }

    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        Self { data }
    }
}

impl<Buf: MemRead + Copy> Property<Buf> {
    pub fn get_name(&self) -> Result<&'_ str, StringBindError> {
        tll::bind::offset_str::<tll::bind::OffsetPtrDefault, Buf>(&self.data, 0)
    }
    pub fn get_value(&self) -> Result<&'_ str, StringBindError> {
        tll::bind::offset_str::<tll::bind::OffsetPtrDefault, Buf>(&self.data, 8)
    }
}
#[derive(Debug)]
pub struct Device<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> Binder<Buf> for Device<Buf> {
    fn bind_view(data: MemOffset<Buf>) -> Result<Self, BindError> {
        if data.mem_size() < 33 {
            return Err(BindError::new_size(33));
        }
        // Pointer
        // Pointer
        // Pointer
        // Pointer
        Ok(Self { data })
    }

    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        Self { data }
    }
}

impl<Buf: MemRead + Copy> Device<Buf> {
    pub fn get_action(&self) -> Action {
        self.data.mem_get_primitive::<Action>(0)
    }
    pub fn get_subsystem(&self) -> Result<&'_ str, StringBindError> {
        tll::bind::offset_str::<tll::bind::OffsetPtrDefault, Buf>(&self.data, 1)
    }
    pub fn get_sysname(&self) -> Result<&'_ str, StringBindError> {
        tll::bind::offset_str::<tll::bind::OffsetPtrDefault, Buf>(&self.data, 9)
    }
    pub fn get_devpath(&self) -> Result<&'_ str, StringBindError> {
        tll::bind::offset_str::<tll::bind::OffsetPtrDefault, Buf>(&self.data, 17)
    }
    pub fn get_properties(
        &self,
    ) -> Result<tll::bind::OffsetPtr<Property<Buf>, tll::bind::OffsetPtrDefault, Buf>, BindError> {
        tll::bind::OffsetPtr::<Property<Buf>, tll::bind::OffsetPtrDefault, Buf>::new(self.data.view(25))
    }
}
impl<Buf: MemRead> MsgId for Device<Buf> {
    const MSGID: i32 = 10;
}
