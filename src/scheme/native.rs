use std::rc::Rc;

use crate::scheme::scheme as S;
pub use crate::scheme::scheme::{PointerVersion, SubTypeRaw, TimeResolution};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
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
        counter: Rc<Field>,
        data: Rc<Field>,
    },
    Pointer {
        version: PointerVersion,
        data: Rc<Field>,
    },
    Message(Rc<Message>),
    Union,
}

impl Default for Type {
    fn default() -> Self {
        Type::Int8
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubType {
    None,
    Enum(Rc<Enum>),
    ByteString,
    TimePoint(TimeResolution),
    Duration(TimeResolution),
    Fixed(usize),
    Bits,
    Unknown(u32),
}

impl Default for SubType {
    fn default() -> Self {
        SubType::None
    }
}

type Options = std::collections::BTreeMap<String, String>;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct EnumValue {
    pub name: String,
    pub value: i64,
}

#[derive(Debug, Default)]
pub struct Enum {
    pub name: String,
    pub size: usize,
    pub enum_type: Type,
    pub options: Options,
    pub values: Vec<EnumValue>,
}

impl PartialEq for Enum {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Eq for Enum {}

#[derive(Debug, Default, Clone)]
pub struct Field {
    pub name: String,
    pub offset: usize,
    pub size: usize,
    pub index: i32,
    pub field_type: Type,
    pub sub_type: SubType,
    pub options: Options,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Eq for Field {}

#[derive(Debug, Default, Clone)]
pub struct Message {
    pub name: String,
    pub msgid: i32,
    pub size: usize,
    pub options: Options,
    pub enums: Vec<Rc<Enum>>,
    pub fields: Vec<Rc<Field>>,
    pub pmap: Option<Rc<Field>>,
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Eq for Message {}

#[derive(Debug, Default, Clone)]
pub struct Scheme {
    pub options: Options,
    pub enums: Vec<Rc<Enum>>,
    pub messages: Vec<Rc<Message>>,
}

impl Enum {
    pub fn get_type(&self) -> &Type {
        &self.enum_type
    }

    fn build(e: &crate::scheme::scheme::Enum) -> Self {
        let mut r = Enum::default();
        r.name = e.name().into();
        r.options = e.options().unwrap_or(Options::new());
        r.values.reserve(e.values().count());
        r.values = e
            .values()
            .map(|v| EnumValue {
                name: v.name().into(),
                value: v.value(),
            })
            .collect();
        r
    }
}
impl Field {
    pub fn get_type(&self) -> &Type {
        &self.field_type
    }

    fn build(s: &Scheme, m: &Message, f: &crate::scheme::scheme::Field) -> Self {
        let mut r = Field::default();
        r.name = f.name().into();
        r.size = f.size();
        r.offset = f.offset();
        r.index = f.index();
        r.options = f.options().unwrap_or(Options::new());
        r.field_type = match f.get_type() {
            S::Type::Int8 => Type::Int8,
            S::Type::Int16 => Type::Int16,
            S::Type::Int32 => Type::Int32,
            S::Type::Int64 => Type::Int64,
            S::Type::UInt8 => Type::UInt8,
            S::Type::UInt16 => Type::UInt16,
            S::Type::UInt32 => Type::UInt32,
            S::Type::UInt64 => Type::UInt64,
            S::Type::Double => Type::Double,
            S::Type::Decimal128 => Type::Decimal128,
            S::Type::Bytes(size) => Type::Bytes(size),
            S::Type::Array {
                capacity: c,
                counter: cnt,
                data,
            } => Type::Array {
                capacity: c,
                counter: Field::build(s, m, &cnt).into(),
                data: Field::build(s, m, &data).into(),
            },
            S::Type::Pointer { version: v, data } => Type::Pointer {
                version: v,
                data: Field::build(s, m, &data).into(),
            },
            S::Type::Message(m) => Type::Message(s.messages.iter().find(|x| x.name == m.name()).unwrap().clone()),
            S::Type::Union => Type::Union,
        };
        r.sub_type = match f.sub_type() {
            S::SubType::None => SubType::None,
            S::SubType::ByteString => SubType::ByteString,
            S::SubType::TimePoint(v) => SubType::TimePoint(v),
            S::SubType::Duration(v) => SubType::Duration(v),
            S::SubType::Bits => SubType::Bits,
            S::SubType::Fixed(v) => SubType::Fixed(v),
            S::SubType::Unknown(v) => SubType::Unknown(v),
            S::SubType::Enum(e) => SubType::Enum(
                m.enums
                    .iter()
                    .find(|x| x.name == e.name())
                    .or_else(|| s.enums.iter().find(|x| x.name == e.name()))
                    .unwrap()
                    .clone(),
            ),
        };
        r
    }
}

impl Message {
    fn build(s: &Scheme, m: &crate::scheme::scheme::Message) -> Self {
        let mut r = Message::default();
        r.name = m.name().into();
        r.msgid = m.msgid();
        r.size = m.size();
        r.options = m.options().unwrap_or(Options::new());
        r.enums = m.enums().map(|x| Enum::build(&x).into()).collect();
        r.fields.reserve(m.fields().count());
        for f in m.fields() {
            r.fields.push(Field::build(&s, &r, &f).into())
        }
        if let Some(f) = m.pmap() {
            r.pmap = r.fields.iter().find(|x| x.name == f.name()).cloned();
        }
        r
    }
}

impl Scheme {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<&crate::scheme::scheme::Scheme> for Scheme {
    fn from(s: &crate::scheme::scheme::Scheme) -> Self {
        let mut r = Scheme::default();
        r.options = s.options().unwrap_or(Options::new());
        r.enums = s.enums().map(|x| Enum::build(&x).into()).collect();
        r.messages.reserve(s.messages().count());
        for m in s.messages() {
            r.messages.push(Message::build(&r, &m).into())
        }
        r
    }
}
