use std::rc::Rc;

use base64::{engine::general_purpose::STANDARD as base64, Engine};
pub use serde_core::ser::Serialize;
use serde_core::ser::{Error, SerializeMap, SerializeSeq, Serializer};

use crate::mem::MemRead;
use crate::scheme::mem::{OffsetPtrDefault, OffsetPtrImpl, OffsetPtrLegacyLong, OffsetPtrLegacyShort};
use crate::scheme::native::*;

pub struct DataMessage<'a> {
    pub data: &'a [u8],
    pub desc: Rc<Message>,
}

pub struct DataField<'a> {
    pub data: &'a [u8],
    pub desc: Rc<Field>,
}

pub struct DataArray<'a> {
    pub data: &'a [u8],
    pub desc: Rc<Field>,
    pub entity: usize,
    pub size: usize,
}

impl<'a> Serialize for DataMessage<'a> {
    fn serialize<S: Serializer>(&self, ser: S) -> std::result::Result<S::Ok, S::Error> {
        if self.data.len() < self.desc.size {
            return Err(Error::custom("Message data too small"));
        }
        let mut s = ser.serialize_map(None)?;
        for f in &self.desc.fields {
            s.serialize_key(&f.name)?;
            s.serialize_value(&DataField {
                data: &self.data[f.offset..],
                desc: f.clone(),
            })?;
        }
        s.end()
    }
}

impl<'a> Serialize for DataArray<'a> {
    fn serialize<S: Serializer>(&self, ser: S) -> std::result::Result<S::Ok, S::Error> {
        let mut s = ser.serialize_seq(Some(self.size))?;
        let mut f = DataField {
            data: self.data,
            desc: self.desc.clone(),
        };
        for _ in 0..self.size {
            s.serialize_element(&f)?;
            f.data = &f.data[self.entity..];
        }
        s.end()
    }
}

#[derive(Debug)]
enum SizeError {
    UnsupportedType,
    NegativeSize,
}

impl std::fmt::Display for SizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SizeError::UnsupportedType => "Unsuppored type",
            SizeError::NegativeSize => "Negative size",
        })
    }
}

fn usize_from_signed(v: i64) -> std::result::Result<usize, SizeError> {
    if v >= 0 {
        Ok(v as usize)
    } else {
        Err(SizeError::NegativeSize)
    }
}

fn read_size(desc: &Field, data: &[u8]) -> std::result::Result<usize, SizeError> {
    match &desc.field_type {
        Type::UInt8 => Ok(data.mem_get_primitive::<u8>(0) as usize),
        Type::UInt16 => Ok(data.mem_get_primitive::<u16>(0) as usize),
        Type::UInt32 => Ok(data.mem_get_primitive::<u32>(0) as usize),
        Type::UInt64 => Ok(data.mem_get_primitive::<u64>(0) as usize),
        Type::Int8 => usize_from_signed(data.mem_get_primitive::<i8>(0) as i64),
        Type::Int16 => usize_from_signed(data.mem_get_primitive::<i16>(0) as i64),
        Type::Int32 => usize_from_signed(data.mem_get_primitive::<i32>(0) as i64),
        Type::Int64 => usize_from_signed(data.mem_get_primitive::<i64>(0) as i64),
        _ => Err(SizeError::UnsupportedType),
    }
}

fn serialize_ptr<'a, Ptr: OffsetPtrImpl, S: Serializer>(
    this: &DataField<'a>,
    data_field: &Rc<Field>,
    ser: S,
) -> std::result::Result<S::Ok, S::Error> {
    let size = Ptr::size(&this.data);
    let offset = Ptr::offset(&this.data);
    let entity = Ptr::entity(&this.data);
    if this.data.len() < offset + size * entity {
        return Err(Error::custom("Offset pointer out of bounds"));
    }
    if this.desc.sub_type == SubType::ByteString {
        ser.serialize_str(if size == 0usize {
            ""
        } else {
            std::str::from_utf8(&this.data[offset..offset + size - 1]).map_err(Error::custom)?
        })
    } else {
        DataArray {
            data: &this.data[offset..],
            desc: data_field.clone(),
            entity,
            size,
        }
        .serialize(ser)
    }
}

impl<'a> Serialize for DataField<'a> {
    fn serialize<S: Serializer>(&self, ser: S) -> std::result::Result<S::Ok, S::Error> {
        match &self.desc.field_type {
            Type::Int8 => self.data.mem_get_primitive::<i8>(0).serialize(ser),
            Type::Int16 => self.data.mem_get_primitive::<i16>(0).serialize(ser),
            Type::Int32 => self.data.mem_get_primitive::<i32>(0).serialize(ser),
            Type::Int64 => self.data.mem_get_primitive::<i64>(0).serialize(ser),
            Type::UInt8 => self.data.mem_get_primitive::<u8>(0).serialize(ser),
            Type::UInt16 => self.data.mem_get_primitive::<u16>(0).serialize(ser),
            Type::UInt32 => self.data.mem_get_primitive::<u32>(0).serialize(ser),
            Type::UInt64 => self.data.mem_get_primitive::<u64>(0).serialize(ser),
            Type::Double => self.data.mem_get_primitive::<f64>(0).serialize(ser),
            Type::Bytes(capacity) => {
                if SubType::ByteString == self.desc.sub_type {
                    let slice = self.data.mem_get_bytestring(0, *capacity);
                    let s = std::str::from_utf8(slice).map_err(Error::custom)?;
                    ser.serialize_str(s)
                } else {
                    ser.serialize_str(&base64.encode(&self.data[..*capacity]))
                }
            }
            Type::Array {
                capacity,
                counter,
                data,
            } => {
                let size = read_size(counter, &self.data[counter.offset..]).map_err(Error::custom)?;
                if size > *capacity {
                    return Err(Error::custom("Array size out of bounds"));
                }
                DataArray {
                    data: &self.data[data.offset..],
                    desc: data.clone(),
                    entity: data.size,
                    size,
                }
                .serialize(ser)
            }
            Type::Pointer { version, data } => match version {
                PointerVersion::Default => serialize_ptr::<OffsetPtrDefault, S>(self, data, ser),
                PointerVersion::LegacyLong => serialize_ptr::<OffsetPtrLegacyLong, S>(self, data, ser),
                PointerVersion::LegacyShort => serialize_ptr::<OffsetPtrLegacyShort, S>(self, data, ser),
            },
            Type::Message(message) => DataMessage {
                data: self.data,
                desc: message.clone(),
            }
            .serialize(ser),
            _ => Err(Error::custom("Unsupported field type")),
        }
    }
}
