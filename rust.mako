#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

pub use tll::scheme::*;

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

DECL_CACHE = set()
options.msgid = 'MSGID'

def field2type(f):
    t = numeric(f.type)
    if t is not None:
	if f.sub_type == f.Sub.Bits:
	    return t #f.name
	elif f.sub_type == f.Sub.Enum:
	    return f.type_enum.name
	elif f.sub_type == f.Sub.Duration:
	    return f"tll::scheme::Duration<{t}, {time_resolution(f)}>"
	elif f.sub_type == f.Sub.TimePoint:
	    return f"tll::scheme::TimePoint<{t}, {time_resolution(f)}>"
        return t
    elif f.type == f.Decimal128:
        return "tll::decimal128::Decimal128"
    elif f.type == f.Bytes:
	if f.sub_type == f.Sub.ByteString:
	    return f"tll::scheme::ByteString<{f.size}>"
        return f"[u8; {f.size}]"
    elif f.type == f.Message:
        return f.type_msg.name
    elif f.type == f.Array:
    	t = field2type(f.type_array)
	ct = field2type(f.count_ptr)
        return f"tll::scheme::Array<{t}, {f.count}, {ct}>"
    elif f.type == f.Pointer:
    	t = field2type(f.type_ptr)
        return f"tll::scheme::OffsetPtr<{t}>"
    raise ValueError(f"Unknown type for field {f.name}: {f.type}")
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
impl Binder for ${e.name} {}
</%def>\
<%def name='bytestring2code(f)'>\
%if False and f'ByteString${f.size}' not in DECL_CACHE:
#[repr(C, packed(1))]
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct ByteString${f.size}
{
	data: [u8; ${f.size}],
}

impl tll::scheme::ByteString for ByteString${f.size}
{
	fn get_data(&self) -> &[u8] { &self.data }
}
impl Binder for ByteString${f.size} {}<% DECL_CACHE.add(f'ByteString${f.size}') %>
%endif
</%def>\
<%def name='field2decl(f)' filter='weaktrim'>
% if f.type == f.Array:
<%call expr='field2decl(f.type_array)'></%call>\
% elif f.type == f.Pointer:
<%call expr='field2decl(f.type_ptr)'></%call>\
% elif f.type == f.Bytes:
<%call expr='bytestring2code(f)'></%call>\
% elif f.sub_type == f.Sub.Bits:
/*
struct ${f.name}: public tll::scheme::Bits<${numeric(f.type)}>
{
% for n,b in sorted(f.bitfields.items(), key=lambda t: (t[1].offset, t[1].size, t[0])):
	auto ${b.name}() const { return get(${b.offset}, ${b.size}); }; void ${b.name}(${"unsigned" if b.size > 1 else "bool"} v) { return set(${b.offset}, ${b.size}, v); };
% endfor
};
*/
% endif
</%def>\
<%def name='field2code(f)'>\
	pub ${keyword(f.name)}: ${field2type(f)},\
</%def>
% for e in scheme.enums.values():
<%call expr='enum2code(e)'></%call>
% endfor
% for msg in scheme.messages:
% for f in msg.fields:
<%call expr='field2decl(f)'></%call>\
% endfor
% endfor
% for msg in scheme.messages:
#[repr(C, packed(1))]
#[ derive( Debug, Clone, Copy ) ]
pub struct ${keyword(msg.name)} {
% for e in msg.enums.values():
<%call expr='enum2code(e)'></%call>
% endfor
% for f in msg.fields:
<%call expr='field2code(f)'></%call>
% endfor
}
% if msg.msgid != 0:
impl MsgId for ${keyword(msg.name)}
{
	const MSGID : i32 = ${msg.msgid};
}
% endif
impl Binder for ${keyword(msg.name)}
{
    fn bind(data: &[u8]) -> Option<<&Self>
    {
        if data.len() < std::mem::size_of::<Self>() { return None; }
% for f in msg.fields:
	<${field2type(f)} as Binder>::bind(&data[${f.offset}..])?; // ${f.name}
% endfor
        Some(unsafe { bind_unchecked::<Self>(data) })
    }
}

% endfor
