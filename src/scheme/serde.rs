use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;

use base64::{engine::general_purpose::STANDARD as base64, Engine};
use chrono::Timelike;
pub use serde_core::ser::Serialize;
use serde_core::ser::{Error, SerializeMap, SerializeSeq, Serializer};

use crate::decimal128::{Decimal128, Unpacked128};
use crate::mem::MemRead;
use crate::scheme::chrono as C;
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

fn res_to_str(res: TimeResolution) -> &'static str {
    match res {
        TimeResolution::Ns => "ns",
        TimeResolution::Us => "us",
        TimeResolution::Ms => "ms",
        TimeResolution::Second => "s",
        TimeResolution::Minute => "m",
        TimeResolution::Hour => "h",
        TimeResolution::Day => "d",
    }
}

fn serialize_integer<
    'a,
    T: Serialize + std::convert::TryInto<i64> + Copy + std::fmt::Display + std::ops::Div + C::Integer,
    S: Serializer,
>(
    v: T,
    field: &Field,
    ser: S,
) -> std::result::Result<S::Ok, S::Error> {
    match &field.sub_type {
        SubType::Enum(e) => {
            let iv: i64 = v.try_into().map_err(|_| S::Error::custom("Integer overflow"))?;
            if let Some(ev) = e.values.iter().find(|x| x.value == iv) {
                ser.serialize_str(&ev.name)
            } else {
                v.serialize(ser)
            }
        }
        SubType::Duration(res) => ser.serialize_str(&format!("{}{}", v, res_to_str(*res))),
        SubType::TimePoint(res) => {
            let dt = match res {
                TimeResolution::Ns => C::TimePoint::<T, C::Nano>::new_raw(v).as_datetime(),
                TimeResolution::Us => C::TimePoint::<T, C::Micro>::new_raw(v).as_datetime(),
                TimeResolution::Ms => C::TimePoint::<T, C::Milli>::new_raw(v).as_datetime(),
                TimeResolution::Second => C::TimePoint::<T, C::Ratio1>::new_raw(v).as_datetime(),
                TimeResolution::Minute => C::TimePoint::<T, C::RatioMinute>::new_raw(v).as_datetime(),
                TimeResolution::Hour => C::TimePoint::<T, C::RatioHour>::new_raw(v).as_datetime(),
                TimeResolution::Day => C::TimePoint::<T, C::RatioDay>::new_raw(v).as_datetime(),
            }
            .map_err(S::Error::custom)?
            .naive_utc();
            let mut buf = Vec::<u8>::with_capacity(32);
            write!(buf, "{}", dt.date().format("%Y-%m-%d")).map_err(S::Error::custom)?;
            let time = dt.time();
            if time.num_seconds_from_midnight() != 0 || time.nanosecond() != 0 {
                write!(buf, "T{}", time.format("%H:%M:%S")).map_err(S::Error::custom)?;
                if time.nanosecond() != 0 {
                    write!(buf, "{}", match res {
                        TimeResolution::Ns => time.format("%.9f"),
                        TimeResolution::Us => time.format("%.6f"),
                        TimeResolution::Ms => time.format("%.3f"),
                        _ => time.format("%.f"),
                    }).map_err(S::Error::custom)?;
                }
            }
            ser.serialize_str(unsafe { str::from_utf8_unchecked(&buf[..]) })
        }
        _ => v.serialize(ser),
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
            Type::Int8 => serialize_integer(self.data.mem_get_primitive::<i8>(0), self.desc.deref(), ser),
            Type::Int16 => serialize_integer(self.data.mem_get_primitive::<i16>(0), self.desc.deref(), ser),
            Type::Int32 => serialize_integer(self.data.mem_get_primitive::<i32>(0), self.desc.deref(), ser),
            Type::Int64 => serialize_integer(self.data.mem_get_primitive::<i64>(0), self.desc.deref(), ser),
            Type::UInt8 => serialize_integer(self.data.mem_get_primitive::<u8>(0), self.desc.deref(), ser),
            Type::UInt16 => serialize_integer(self.data.mem_get_primitive::<u16>(0), self.desc.deref(), ser),
            Type::UInt32 => serialize_integer(self.data.mem_get_primitive::<u32>(0), self.desc.deref(), ser),
            Type::UInt64 => serialize_integer(self.data.mem_get_primitive::<u64>(0), self.desc.deref(), ser),
            Type::Double => self.data.mem_get_primitive::<f64>(0).serialize(ser),
            Type::Decimal128 => {
                let mut buf = String::new();
                match self.data.mem_get_primitive::<Decimal128>(0).unpack() {
                    Unpacked128::Value(s, m, e) => {
                        if s {
                            buf += "-"
                        }
                        buf += &m.to_string();
                        buf += ".0E";
                        buf += &e.to_string();
                        &buf
                    }
                    Unpacked128::Inf => "Inf",
                    Unpacked128::NegInf => "-Inf",
                    Unpacked128::NaN => "NaN",
                    Unpacked128::SNaN => "sNan",
                }
                .serialize(ser)
            }
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
