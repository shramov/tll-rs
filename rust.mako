#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

pub use tll::bind::*;

pub const SCHEME_STRING : &str = "${scheme.dump('yamls+gz')}";

<%!
def weaktrim(text):
    text = text.lstrip('\n')
    r = text.strip()
    if r == '': return r
    return text
%>\
<%
NUMERIC = {
    S.Type.Int8: 'i8',
    S.Type.Int16: 'i16',
    S.Type.Int32: 'i32',
    S.Type.Int64: 'i64',
    S.Type.UInt8: 'u8',
    S.Type.UInt16: 'u16',
    S.Type.UInt32: 'u32',
    S.Type.UInt64: 'u64',
    S.Type.Double: 'f64',
}

def numeric(t):
    return NUMERIC.get(t, None)

RESOLUTION = {
    S.chrono.Resolution.ns: 'tll::scheme::Nano',
    S.chrono.Resolution.us: 'tll::scheme::Micro',
    S.chrono.Resolution.ms: 'tll::scheme::Milli',
    S.chrono.Resolution.second: 'tll::scheme::Ratio1',
    S.chrono.Resolution.minute: 'tll::scheme::RatioMinute',
    S.chrono.Resolution.hour: 'tll::scheme::RatioHour',
    S.chrono.Resolution.day: 'tll::scheme::RatioDay',
}
def time_resolution(f):
    r = RESOLUTION.get(f.time_resolution, None)
    if r is None:
        raise ValueError(f"Unknown time resolution for field {f.name}: {f.time_resolution}")
    return r

KEYWORDS = {'type': 'type_'}
def keyword(n):
    return KEYWORDS.get(n, n)

OFFSET_PTR_VERSION = {
    S.OffsetPtrVersion.Default: 'tll::bind::OffsetPtrDefault',
    S.OffsetPtrVersion.LegacyShort: 'tll::bind::OffsetPtrLegacyShort',
    S.OffsetPtrVersion.LegacyLong: 'tll::bind::OffsetPtrLegacyLong',
}

def has_pointer_field(f):
    if f.type == f.Pointer:
        return True
    elif f.type == f.Array:
        return has_pointer_field(f.type_array)
    elif f.type == f.Message:
        return has_pointer(f.type_msg)
    elif f.type == f.Union:
        return any([has_pointer(f) for f in f.type_union.fields])
    return False

def has_pointer(msg):
    for f in msg.fields:
        if has_pointer_field(f): return True
    return False

DECL_CACHE = set()
options.msgid = 'MSGID'

def primitive(f):
    if numeric(f.type) is not None:
        return True
    if f.type == f.Type.Decimal128:
        return True
    return False

def _field2type(f):
    t = numeric(f.type)
    if t is not None:
        if f.sub_type == f.Sub.Bits:
            return t, None #f.name
        elif f.sub_type == f.Sub.Enum:
            return f.type_enum.name, None
        elif f.sub_type == f.Sub.Duration:
            return f"tll::scheme::Duration<{t}, {time_resolution(f)}>", None
        elif f.sub_type == f.Sub.TimePoint:
            return f"tll::scheme::TimePoint<{t}, {time_resolution(f)}>", None
        return t, None
    elif f.type == f.Decimal128:
        return "tll::decimal128::Decimal128", None
    elif f.type == f.Bytes:
        if f.sub_type == f.Sub.ByteString:
            return f' &\'_ str', 'StringBindError'
        return f"tll::bind::Bytes<{f.size}, Buf>", None
    elif f.type == f.Message:
        return f"{f.type_msg.name}<Buf>", None
    elif f.type == f.Array:
        t = field2type(f.type_array)
        ct = field2type(f.count_ptr)
        return f"tll::bind::Array<{ct}, {t}, {f.count}, MemOffset<Buf>>", None
    elif f.type == f.Pointer:
        if f.sub_type == f.Sub.ByteString:
            return f' &\'_ str', 'StringBindError'
        t = field2type(f.type_ptr)
        return f"tll::bind::OffsetPtr<{t}, {OFFSET_PTR_VERSION[f.offset_ptr_version]}, Buf>", "BindError"
    elif f.type == f.Union:
    	return f"{f.type_union.name}<Buf>", "UnionBindError"
    raise ValueError(f"Unknown type for field {f.name}: {f.type}")

def field2type(f):
    t, err = _field2type(f)
    if err is None:
        return t
    return f'Result<{t}, {err}>'
%>\
<%def name='enum2code(e)'>\
#[repr(${numeric(e.type)})]
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum ${e.name}
{
% for n,v in sorted(e.items(), key=lambda t: (t[1], t[0])):
    ${keyword(n)} = ${v},
% endfor
}
impl BinderCopy for ${e.name} { type Target = Self; }
</%def>\
<%def name='union2code(u, prefix="")'>\
#[ derive( Debug ) ]
pub enum ${u.name}<Buf: MemRead>
{
% for f in u.fields:
        ${f.name}(${_field2type(f)[0]}),
% endfor
}

impl <Buf: MemRead + Copy> UnionType<Buf> for ${u.name}<Buf> {
    fn bind_index(index: usize, data: MemOffset<Buf>) -> Result<Self, UnionBindError> {
        match index {
% for i,f in enumerate(u.fields):
            ${i} => Ok(Self::${f.name}(
% if primitive(f):
	    data.mem_get_primitive::<${field2type(f)}>(0)
% elif f.type == f.Type.Message:
	    ${f.type_msg.name}::bind_view(data)?
% endif
	    )),
% endfor
            _ => Err(UnionBindError::UnionError(index)),
        }
    }
}
</%def>\
<%def name='field2decl(f, prefix="")' filter='weaktrim'>
% if f.type == f.Array:
<%call expr='field2decl(f.type_array)'></%call>\
% elif f.type == f.Pointer:
<%call expr='field2decl(f.type_ptr)'></%call>\
% elif f.type == f.Bytes:
% elif f.sub_type == f.Sub.Bits:
% elif f.type == f.Union:
<%call expr='union2code(f.type_union, prefix=prefix)'></%call>\
% for uf in f.type_union.fields:
<%call expr='field2decl(uf, prefix=prefix)'></%call>\
% endfor
% endif
</%def>\
% for e in scheme.enums.values():
<%call expr='enum2code(e)'></%call>
% endfor
% for u in scheme.unions.values():
<%call expr='union2code(u)'></%call>
% endfor
% for msg in scheme.messages:
% for e in msg.enums.values():
<%call expr='enum2code(e)'></%call>
% endfor
% for f in msg.fields:
<%call expr='field2decl(f, prefix=f"{msg.name}_")'></%call>\
% endfor
% endfor
% for msg in scheme.messages:
#[ derive( Debug ) ]
pub struct ${keyword(msg.name)}<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> Binder<Buf> for ${keyword(msg.name)}<Buf> {
    fn bind_view(data: MemOffset<Buf>) -> Result<Self, BindError> {
        if data.mem_size() < ${msg.size} { return Err(BindError::new_size(${msg.size})); }
% for f in msg.fields:
% if f.type == f.Type.Pointer:
        // Pointer
% elif f.type == f.Type.Array:
        // Array
% elif f.type == f.Type.Message:
        ${keyword(f.type_msg.name)}::bind_view(data.view(${f.offset}))?;
% elif f.type == f.Type.Union:
        // Union
% endif
% endfor
        Ok(Self { data })
    }

    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        Self { data }
    }
}

impl<Buf: MemRead + Copy> ${keyword(msg.name)}<Buf> {
    pub fn meta_size() -> usize { ${msg.size} }

% for f in msg.fields:
    pub fn get_${keyword(f.name)}(&self) -> ${field2type(f)} {
% if primitive(f):
        self.data.mem_get_primitive::<${field2type(f)}>(${f.offset})
% elif f.type == f.Type.Message:
        ${keyword(f.type_msg.name)}::<Buf> { data: self.data.view(${f.offset})}
% elif f.type == f.Type.Bytes:
% if f.sub_type == f.Sub.ByteString:
        tll::bind::byte_str(&self.data, ${f.offset}, ${f.size})
% else:
        tll::bind::Bytes::<${f.size}, Buf>::bind_unchecked(self.data.view(${f.offset}))
% endif
% elif f.type == f.Type.Array:
        tll::bind::Array::<${field2type(f.count_ptr)}, ${field2type(f.type_array)}, ${f.count}, MemOffset<Buf>>::new(self.data.view(${f.offset}))
% elif f.type == f.Type.Pointer and f.sub_type == f.Sub.ByteString:
        tll::bind::offset_str::<${OFFSET_PTR_VERSION[f.offset_ptr_version]}, Buf>(&self.data, ${f.offset})
% elif f.type == f.Type.Pointer:
        tll::bind::OffsetPtr::<${field2type(f.type_ptr)}, ${OFFSET_PTR_VERSION[f.offset_ptr_version]}, Buf>::new(self.data.view(${f.offset}))
% elif f.type == f.Type.Union:
	tll::bind::union_bind::<${f.type_union.name}<Buf>, u8, Buf>(self.data, ${f.offset})
% endif
    }
% endfor
}

impl<Buf: MemWrite> ${keyword(msg.name)}<Buf> {
% for f in msg.fields:
% if primitive(f):
    pub fn set_${keyword(f.name)}(&mut self, v: ${field2type(f)}) {
        self.data.mem_set_primitive::<${field2type(f)}>(${f.offset}, v)
    }
% elif f.type == f.Type.Bytes:
% if f.sub_type == f.Sub.ByteString:
    pub fn set_${keyword(f.name)}(&mut self, v: &str) {
        self.data.mem_set_bytes(${f.offset}, ${f.size}, v.as_bytes())
% else:
    pub fn set_${keyword(f.name)}(&mut self, v: &[u8]) {
        self.data.mem_set_bytes(${f.offset}, ${f.size}, v)
% endif
    }
% elif f.type == f.Type.Message:
    pub fn mut_${keyword(f.name)}(&mut self) -> ${keyword(f.name)}::< &mut Buf> {
    	${keyword(f.name)}::bind_unchecked(self.data.reborrow().view(${f.offset}))
    }
% elif f.type == f.Type.Array:
    pub fn mut_${keyword(f.name)}(&mut self) -> tll::bind::Array::<${field2type(f.count_ptr)}, ${field2type(f.type_array)}, ${f.count}, MemOffset< &mut Buf>> {
        tll::bind::Array::<${field2type(f.count_ptr)}, ${field2type(f.type_array)}, ${f.count}, MemOffset< &mut Buf>>::new(self.data.reborrow().view(${f.offset}))
    }
% endif
% endfor
}
% if msg.msgid != 0:
impl<Buf: MemRead> MsgId for ${keyword(msg.name)}<Buf>
{
        const MSGID : i32 = ${msg.msgid};
}
% endif
% endfor
