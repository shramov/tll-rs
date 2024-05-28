use tll_sys::scheme::*;

use crate::error::*;

use std::ffi::CStr;
/*
use std::ffi::CString;
*/
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeRaw {
    Int8 = TLL_SCHEME_FIELD_INT8 as isize,
    Int16 = TLL_SCHEME_FIELD_INT16 as isize,
    Int32 = TLL_SCHEME_FIELD_INT32 as isize,
    Int64 = TLL_SCHEME_FIELD_INT64 as isize,
    UInt8 = TLL_SCHEME_FIELD_UINT8 as isize,
    UInt16 = TLL_SCHEME_FIELD_UINT16 as isize,
    UInt32 = TLL_SCHEME_FIELD_UINT32 as isize,
    UInt64 = TLL_SCHEME_FIELD_UINT64 as isize,
    Double = TLL_SCHEME_FIELD_DOUBLE as isize,
    Decimal128 = TLL_SCHEME_FIELD_DECIMAL128 as isize,
    Bytes = TLL_SCHEME_FIELD_BYTES as isize,
    Array = TLL_SCHEME_FIELD_ARRAY as isize,
    Pointer = TLL_SCHEME_FIELD_POINTER as isize,
    Message = TLL_SCHEME_FIELD_MESSAGE as isize,
    Union = TLL_SCHEME_FIELD_UNION as isize,
}

impl From<tll_scheme_field_type_t> for TypeRaw {
    fn from(t: tll_scheme_field_type_t) -> Self {
        match t {
            TLL_SCHEME_FIELD_INT8 => TypeRaw::Int8,
            TLL_SCHEME_FIELD_INT16 => TypeRaw::Int16,
            TLL_SCHEME_FIELD_INT32 => TypeRaw::Int32,
            TLL_SCHEME_FIELD_INT64 => TypeRaw::Int64,
            TLL_SCHEME_FIELD_UINT8 => TypeRaw::UInt8,
            TLL_SCHEME_FIELD_UINT16 => TypeRaw::UInt16,
            TLL_SCHEME_FIELD_UINT32 => TypeRaw::UInt32,
            TLL_SCHEME_FIELD_UINT64 => TypeRaw::UInt64,
            TLL_SCHEME_FIELD_DOUBLE => TypeRaw::Double,
            TLL_SCHEME_FIELD_DECIMAL128 => TypeRaw::Decimal128,
            TLL_SCHEME_FIELD_BYTES => TypeRaw::Bytes,
            TLL_SCHEME_FIELD_ARRAY => TypeRaw::Array,
            TLL_SCHEME_FIELD_POINTER => TypeRaw::Pointer,
            TLL_SCHEME_FIELD_MESSAGE => TypeRaw::Message,
            TLL_SCHEME_FIELD_UNION => TypeRaw::Union,
            _ => panic!("Invalid type {:?}", t),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TimeResolution {
    Ns = TLL_SCHEME_TIME_NS as isize,
    Us = TLL_SCHEME_TIME_US as isize,
    Ms = TLL_SCHEME_TIME_MS as isize,
    Second = TLL_SCHEME_TIME_SECOND as isize,
    Minute = TLL_SCHEME_TIME_MINUTE as isize,
    Hour = TLL_SCHEME_TIME_HOUR as isize,
    Day = TLL_SCHEME_TIME_DAY as isize,
}

impl From<tll_scheme_time_resolution_t> for TimeResolution {
    fn from(t: tll_scheme_time_resolution_t) -> Self {
        match t {
            TLL_SCHEME_TIME_NS => TimeResolution::Ns,
            TLL_SCHEME_TIME_US => TimeResolution::Us,
            TLL_SCHEME_TIME_MS => TimeResolution::Ms,
            TLL_SCHEME_TIME_SECOND => TimeResolution::Second,
            TLL_SCHEME_TIME_MINUTE => TimeResolution::Minute,
            TLL_SCHEME_TIME_HOUR => TimeResolution::Hour,
            TLL_SCHEME_TIME_DAY => TimeResolution::Day,
            _ => panic!("Invalid time resolution {:?}", t),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PointerVersion {
    Default = TLL_SCHEME_OFFSET_PTR_DEFAULT as isize,
    LegacyShort = TLL_SCHEME_OFFSET_PTR_LEGACY_SHORT as isize,
    LegacyLong = TLL_SCHEME_OFFSET_PTR_LEGACY_LONG as isize,
}

impl From<tll_scheme_offset_ptr_version_t> for PointerVersion {
    fn from(t: tll_scheme_offset_ptr_version_t) -> Self {
        match t {
            TLL_SCHEME_OFFSET_PTR_DEFAULT => PointerVersion::Default,
            TLL_SCHEME_OFFSET_PTR_LEGACY_SHORT => PointerVersion::LegacyShort,
            TLL_SCHEME_OFFSET_PTR_LEGACY_LONG => PointerVersion::LegacyLong,
            _ => panic!("Invalid pointer version {:?}", t),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type<'a> {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Double,
    Decimal128,
    Bytes(usize),
    Array {
        capacity: usize,
        counter: Field<'a>,
        data: Field<'a>,
    },
    Pointer {
        version: PointerVersion,
        data: Field<'a>,
    },
    Message(Message<'a>),
    Union,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SubTypeRaw {
    None,
    Enum,
    ByteString,
    TimePoint,
    Duration,
    Fixed,
    Bits,
    Unknown(u32),
}

impl From<tll_scheme_sub_type_t> for SubTypeRaw {
    fn from(t: tll_scheme_sub_type_t) -> Self {
        match t {
            TLL_SCHEME_SUB_NONE => SubTypeRaw::None,
            TLL_SCHEME_SUB_ENUM => SubTypeRaw::Enum,
            TLL_SCHEME_SUB_BYTE_STRING => SubTypeRaw::ByteString,
            TLL_SCHEME_SUB_TIME_POINT => SubTypeRaw::TimePoint,
            TLL_SCHEME_SUB_DURATION => SubTypeRaw::Duration,
            TLL_SCHEME_SUB_FIXED_POINT => SubTypeRaw::Fixed,
            TLL_SCHEME_SUB_BITS => SubTypeRaw::Bits,
            v => SubTypeRaw::Unknown(v),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SubType {
    None,
    Enum,
    ByteString,
    TimePoint(TimeResolution),
    Duration(TimeResolution),
    Fixed(usize),
    Bits,
    Unknown(u32),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Scheme {
    ptr: *const tll_scheme_t,
}

impl From<*const tll_scheme_t> for Scheme {
    fn from(ptr: *const tll_scheme_t) -> Self {
        assert!(!ptr.is_null());
        unsafe { tll_scheme_ref(ptr) };
        Scheme { ptr: ptr }
    }
}

impl Drop for Scheme {
    fn drop(&mut self) {
        unsafe { tll_scheme_unref(self.ptr) };
        self.ptr = std::ptr::null();
    }
}

trait WithNext {
    unsafe fn get_next_unchecked(value: *const Self) -> *const Self;
    unsafe fn get_name_unchecked(value: *const Self) -> *const i8;

    #[inline(always)]
    fn get_next(value: *const Self) -> *const Self {
        if value.is_null() {
            value
        } else {
            unsafe { Self::get_next_unchecked(value) }
        }
    }

    #[inline(always)]
    fn get_name<'a>(value: *const Self) -> &'a str {
        let name = unsafe { Self::get_name_unchecked(value) };
        if name.is_null() {
            ""
        } else {
            unsafe { CStr::from_ptr(name) }.to_str().unwrap()
        }
    }
}

impl WithNext for tll_scheme_message_t {
    #[inline(always)]
    unsafe fn get_next_unchecked(value: *const Self) -> *const Self {
        (*value).next
    }

    #[inline(always)]
    unsafe fn get_name_unchecked(value: *const Self) -> *const i8 {
        (*value).name
    }
}

impl WithNext for tll_scheme_field_t {
    #[inline(always)]
    unsafe fn get_next_unchecked(value: *const Self) -> *const Self {
        (*value).next
    }

    #[inline(always)]
    unsafe fn get_name_unchecked(value: *const Self) -> *const i8 {
        (*value).name
    }
}

#[derive(Debug, Copy)]
struct Pointer<'a, T> {
    ptr: *const T,
    phantom: PhantomData<&'a ()>,
}

impl<'a, T> Clone for Pointer<'a, T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> PartialEq for Pointer<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<'a, T> Eq for Pointer<'a, T> {}

impl<'a, T> Pointer<'a, T>
where
    T: WithNext,
{
    fn new(ptr: *const T) -> Pointer<'a, T> {
        Pointer {
            ptr: ptr,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    fn next_opt(&self) -> Option<Pointer<'a, T>> {
        let next = T::get_next(self.ptr);
        if next.is_null() {
            None
        } else {
            Some(Pointer::new(next))
        }
    }

    #[inline(always)]
    fn next_iter(&mut self) -> Option<Self> {
        if self.ptr.is_null() {
            None
        } else {
            let r = Some(self.clone());
            self.ptr = T::get_next(self.ptr);
            r
        }
    }
}

impl Scheme {
    pub fn new(url: &str) -> Result<Self> {
        let ptr = unsafe { tll_scheme_load(url.as_ptr() as *const c_char, url.len() as c_int) };
        if ptr.is_null() {
            Err(Error::from("Failed to load scheme"))
        } else {
            Ok(Scheme { ptr: ptr })
        }
    }

    pub fn as_ptr(&self) -> *const tll_scheme_t {
        self.ptr
    }

    pub fn copy(&self) -> Self {
        let ptr = unsafe { tll_scheme_copy(self.ptr) };
        assert!(!ptr.is_null());
        Scheme { ptr: ptr }
    }

    pub fn messages(&self) -> MessageIter {
        MessageIter {
            data: Pointer::new(unsafe { (*self.ptr).messages }),
        }
    }

    pub fn message(&self, id: i32) -> Option<Message> {
        for m in self.messages() {
            if m.msgid() == id {
                return Some(m);
            };
        }
        None
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Message<'a> {
    data: Pointer<'a, tll_scheme_message_t>,
}

impl<'a> Message<'a> {
    #[inline(always)]
    fn from_pointer(ptr: Pointer<'a, tll_scheme_message_t>) -> Self {
        Self { data: ptr }
    }
    pub fn as_ptr(&self) -> *const tll_scheme_message_t {
        self.data.ptr
    }

    #[inline(always)]
    pub fn next(&self) -> Option<Message<'a>> {
        self.data.next_opt().map(Self::from_pointer)
    }

    #[inline(always)]
    pub fn name(&self) -> &'a str {
        WithNext::get_name(self.data.ptr)
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        unsafe { (*self.data.ptr).size }
    }

    #[inline(always)]
    pub fn msgid(&self) -> i32 {
        unsafe { (*self.data.ptr).msgid }
    }

    #[inline(always)]
    pub fn fields(&self) -> FieldIter {
        FieldIter {
            data: Pointer::new(unsafe { (*self.data.ptr).fields }),
        }
    }

    #[inline(always)]
    pub fn pmap(&'a self) -> Option<Field<'a>> {
        let pmap = unsafe { (*self.data.ptr).pmap };
        if pmap.is_null() {
            None
        } else {
            Some(Field::from_pointer(Pointer::new(pmap)))
        }
    }
}

#[derive(Debug)]
pub struct DetachedMessage {
    _scheme: std::rc::Rc<Scheme>,
    ptr: *const tll_scheme_message_t,
}

impl DetachedMessage {
    pub fn new<'a>(scheme: std::rc::Rc<Scheme>, msg: &Message<'a>) -> Self {
        Self {
            _scheme: scheme,
            ptr: msg.as_ptr(),
        }
    }

    pub fn message<'a>(&'a self) -> Message<'a> {
        Message::from_pointer(Pointer::new(self.ptr))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MessageIter<'a> {
    data: Pointer<'a, tll_scheme_message_t>,
}

impl<'a> std::iter::Iterator for MessageIter<'a> {
    type Item = Message<'a>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.data.next_iter().map(Self::Item::from_pointer)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Field<'a> {
    data: Pointer<'a, tll_scheme_field_t>,
}

impl<'a> Field<'a> {
    #[inline(always)]
    fn from_pointer(ptr: Pointer<'a, tll_scheme_field_t>) -> Self {
        Self { data: ptr }
    }

    #[inline(always)]
    pub fn next(&self) -> Option<Field<'a>> {
        self.data.next_opt().map(Self::from_pointer)
    }

    #[inline(always)]
    pub fn name(&self) -> &'a str {
        WithNext::get_name(self.data.ptr)
    }

    #[inline(always)]
    pub fn type_raw(&self) -> TypeRaw {
        TypeRaw::from(unsafe { (*self.data.ptr).type_ })
    }

    #[inline(always)]
    pub fn get_type(&self) -> Type {
        match self.type_raw() {
            TypeRaw::Int8 => Type::Int8,
            TypeRaw::Int16 => Type::Int16,
            TypeRaw::Int32 => Type::Int32,
            TypeRaw::Int64 => Type::Int64,
            TypeRaw::UInt8 => Type::UInt8,
            TypeRaw::UInt16 => Type::UInt16,
            TypeRaw::UInt32 => Type::UInt32,
            TypeRaw::UInt64 => Type::UInt64,
            TypeRaw::Double => Type::Double,
            TypeRaw::Decimal128 => Type::Decimal128,
            TypeRaw::Bytes => Type::Bytes(self.size()),
            TypeRaw::Message => unsafe {
                Type::Message(Message::from_pointer(Pointer::new(
                    (*self.data.ptr).__bindgen_anon_1.type_msg,
                )))
            },
            TypeRaw::Array => unsafe {
                Type::Array {
                    capacity: (*self.data.ptr).__bindgen_anon_1.__bindgen_anon_2.count as usize,
                    counter: Field::from_pointer(Pointer::new(
                        (*self.data.ptr).__bindgen_anon_1.__bindgen_anon_2.count_ptr,
                    )),
                    data: Field::from_pointer(Pointer::new(
                        (*self.data.ptr).__bindgen_anon_1.__bindgen_anon_2.type_array,
                    )),
                }
            },
            TypeRaw::Pointer => unsafe {
                Type::Pointer {
                    version: PointerVersion::from(
                        (*self.data.ptr).__bindgen_anon_1.__bindgen_anon_1.offset_ptr_version,
                    ),
                    data: Field::from_pointer(Pointer::new(
                        (*self.data.ptr).__bindgen_anon_1.__bindgen_anon_1.type_ptr,
                    )),
                }
            },
            TypeRaw::Union => Type::Union,
        }
    }

    #[inline(always)]
    pub fn offset(&self) -> usize {
        unsafe { (*self.data.ptr).offset }
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        unsafe { (*self.data.ptr).size }
    }

    #[inline(always)]
    pub fn index(&self) -> i32 {
        unsafe { (*self.data.ptr).index }
    }

    #[inline(always)]
    pub fn sub_type_raw(&self) -> SubTypeRaw {
        SubTypeRaw::from(unsafe { (*self.data.ptr).sub_type })
    }

    #[inline(always)]
    pub fn sub_type(&self) -> SubType {
        match self.sub_type_raw() {
            SubTypeRaw::Unknown(v) => SubType::Unknown(v),
            SubTypeRaw::None => SubType::None,
            SubTypeRaw::Enum => SubType::Enum,
            SubTypeRaw::Bits => SubType::Bits,
            SubTypeRaw::ByteString => SubType::ByteString,
            SubTypeRaw::Fixed => SubType::Fixed(unsafe { (*self.data.ptr).__bindgen_anon_1.fixed_precision as usize }),
            SubTypeRaw::Duration => SubType::Duration(TimeResolution::from(unsafe {
                (*self.data.ptr).__bindgen_anon_1.time_resolution
            })),
            SubTypeRaw::TimePoint => SubType::TimePoint(TimeResolution::from(unsafe {
                (*self.data.ptr).__bindgen_anon_1.time_resolution
            })),
        }
    }

    pub fn type_msg(&self) -> Option<Message<'a>> {
        if self.type_raw() == TypeRaw::Message {
            unsafe { self.type_msg_unchecked() }
        } else {
            None
        }
    }

    pub unsafe fn type_msg_unchecked(&self) -> Option<Message<'a>> {
        let ptr = (*self.data.ptr).__bindgen_anon_1.type_msg;
        if ptr.is_null() {
            None
        } else {
            Some(Message::from_pointer(Pointer::new(ptr)))
        }
    }

    pub fn type_ptr(&self) -> Option<Field<'a>> {
        if self.type_raw() == TypeRaw::Pointer {
            unsafe { self.type_ptr_unchecked() }
        } else {
            None
        }
    }

    pub unsafe fn type_ptr_unchecked(&self) -> Option<Field<'a>> {
        let ptr = (*self.data.ptr).__bindgen_anon_1.__bindgen_anon_1.type_ptr;
        if ptr.is_null() {
            None
        } else {
            Some(Field::from_pointer(Pointer::new(ptr)))
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FieldIter<'a> {
    data: Pointer<'a, tll_scheme_field_t>,
}

impl<'a> std::iter::Iterator for FieldIter<'a> {
    type Item = Field<'a>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.data.next_iter().map(Self::Item::from_pointer)
    }
}
