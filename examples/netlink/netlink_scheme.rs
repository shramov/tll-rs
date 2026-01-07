#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

pub use tll::bind::*;

pub const SCHEME_STRING : &str = "yamls+gz://eJzlV0tv4zYQvu+v0E0XB/Db2dxkO4sY9Quys01RLAxaohMiMimIVLJu4P/eGUrWk1l7s2iBoieT5sd5fcOZ0ZXFyZ7eWLb9ybJEqJjg8sZ6s70wvMITGRKP2nDOqQoYf95I74nuqX0EOOXxXt7AwrJsx8OrAHxThxDkMa6uGxqB0sY0oArFtBqWPaevsGoej8lNdw0X3rvozP8YOas1HHfh5nDqjH67W0xvYd/Hvbtwxul5B/bTxciZwroN65nzgPr08n66nqSwHhrg4KrVhOXSXdxNhhPcg1Z7fecufof1Z1jfz093Wnrn3jqjO2eolQ/0P6vl7Qg9gc2De7taTL/iWaulPYu5DmXi42Tp8AP6qP+9sf58S6POwpeu3bAS1+0YnO+07WPDKpz38/PtQdFW3z5+QwUkYETSVMNVdmOyNEmsgwxiqyCuaLRD+qvIRjFTVEKfLVXE+KMNzl+dcmoouG8XE2XHaODXTA73JCybXNWhEXB2oNI+Vu3cC5+Wr1+bLUQzMLhZck2HaSJh9r7QIfGe4zAlfBgJ4ntEqjS3ZtfNdscZp5nouinxay0Ds+phgf+1a+YRLXsjA/JCa8SU7EyWJHjXU+JvJLwkT50lGJAhiRSn0WZPvDKB/bNaSwSutOHnWZSKqA/yoCOUBjQnoU40YxvUEssPqUHJKZlj8crTOvGFsOD0xsO0LmX+T6HioevMx3pRKXnlupWkba7tVj3RKBU84TvG2ZbAawD9qHUqRLhN7BkM8I+54Cis3+t1ML8W6W3co8HrmHOKdg76WKe+Cuanpz0dpi1TJ7O+BORRlu1ClRqRF557fHFit5NU6bhL9hfFmOelJ38AObBlAo7pNn4sgNomUOZwjuuYcEsB9q6F/ilguybsXKwjYI9GsoDsmZAuhA+LUw7rmwU6UTEuA6OFkdgz6RVg1yaYEwSzOFCsgPtsws0gwkB1IcZGNlZp+chQRiq0xipnRj6WIlIryKgCzsiHEysxoz4jRaSRjfEBFqwYl1aBi2/GJ4pJadfbJT47eDPC1DE5PpTK8y9YscWWkx3rBlQ5rVbivMYlffUf6VEkGY8yAem4VIXpuGQgXWBMkE1EXmvGVIGM+/R7jjL3Cfz52fZelhCHNTYqiJ0uSBkoqU9VUBAQ349+tlFVZEDaMEybTEqWSIWa7opY0e6pqreb71J+KWdkG9Qb+4+ZdY3UCrb7NS58qaDdy+dzjPjy/AQhI+8iWYAzyCpHu3+Kdud/H+13Zu0PRTuTlUXbwTeUxrr7y7H+lypIGNEd+37O83J5SD6oCp7PKXt8Ornea5ZHopUeTWujWmUkmnBP7EP9oXpmNHIp8Z7SRPzhaASKSyBjK4aPY3I4OxdFYluUZGzBOMtS/9wwVJ1yjKPQkkZ7wmlpCBtc3s7/C3lX+WBJkuSytLukdx0//Q2PWjC0";

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    New = 0,
    Delete = 1,
}
impl BinderCopy for Action {}

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RType {
    UNSPEC = 0,
    UNICAST = 1,
    LOCAL = 2,
    BROADCAST = 3,
    ANYCAST = 4,
    MULTICAST = 5,
    BLACKHOLE = 6,
    UNREACHABLE = 7,
    PROHIBIT = 8,
    THROW = 9,
    NAT = 10,
    XRESOLVE = 11,
    MAX = 12,
}
impl BinderCopy for RType {}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum mode {
    RR = 0,
    ActiveBackup = 1,
    XOR = 2,
    Broadcast = 3,
    M8023AD = 4,
    TLB = 5,
    ALB = 6,
}
impl BinderCopy for mode {}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum state {
    Active = 0,
    Backup = 1,
}
impl BinderCopy for state {}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum mii_status {
    Up = 0,
    Fail = 1,
    Down = 2,
    Back = 3,
}
impl BinderCopy for mii_status {}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Ether = 1,
    Infiniband = 32,
    Tunnel = 768,
    Loopback = 772,
    Other = 65533,
    None = 65534,
    Void = 65535,
}
impl BinderCopy for Type {}

#[derive(Debug)]
pub struct Bond<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> Binder<Buf> for Bond<Buf> {
    fn bind_view(data: MemOffset<Buf>) -> Result<Self, BindError> {
        if data.mem_size() < 17 {
            return Err(BindError::new_size(17));
        }
        Ok(Self { data })
    }

    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        Self { data }
    }
}

impl<Buf: MemRead + Copy> Bond<Buf> {
    pub fn get_pmap(&self) -> u16 {
        self.data.mem_get_primitive::<u16>(0)
    }
    pub fn get_mode(&self) -> mode {
        self.data.mem_get_primitive::<mode>(2)
    }
    pub fn get_active_slave(&self) -> u32 {
        self.data.mem_get_primitive::<u32>(3)
    }
    pub fn get_ad_select(&self) -> u32 {
        self.data.mem_get_primitive::<u32>(7)
    }
    pub fn get_ad_partner_mac(&self) -> &[u8] {
        &self.data.as_mem()[11..11 + 6]
    }
}
#[derive(Debug)]
pub struct BondSlave<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> Binder<Buf> for BondSlave<Buf> {
    fn bind_view(data: MemOffset<Buf>) -> Result<Self, BindError> {
        if data.mem_size() < 2 {
            return Err(BindError::new_size(2));
        }
        Ok(Self { data })
    }

    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        Self { data }
    }
}

impl<Buf: MemRead + Copy> BondSlave<Buf> {
    pub fn get_state(&self) -> state {
        self.data.mem_get_primitive::<state>(0)
    }
    pub fn get_mii_status(&self) -> mii_status {
        self.data.mem_get_primitive::<mii_status>(1)
    }
}
#[derive(Debug)]
pub struct Link<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> Binder<Buf> for Link<Buf> {
    fn bind_view(data: MemOffset<Buf>) -> Result<Self, BindError> {
        if data.mem_size() < 56 {
            return Err(BindError::new_size(56));
        }
        // Union
        Ok(Self { data })
    }

    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        Self { data }
    }
}

impl<Buf: MemRead + Copy> Link<Buf> {
    pub fn get_pmap(&self) -> u16 {
        self.data.mem_get_primitive::<u16>(0)
    }
    pub fn get_action(&self) -> Action {
        self.data.mem_get_primitive::<Action>(2)
    }
    pub fn get_type_(&self) -> Type {
        self.data.mem_get_primitive::<Type>(3)
    }
    pub fn get_type_raw(&self) -> u16 {
        self.data.mem_get_primitive::<u16>(5)
    }
    pub fn get_index(&self) -> i32 {
        self.data.mem_get_primitive::<i32>(7)
    }
    pub fn get_name(&self) -> Result<&'_ str, StringBindError> {
        tll::bind::byte_str(&self.data, 11, 16)
    }
    pub fn get_up(&self) -> u8 {
        self.data.mem_get_primitive::<u8>(27)
    }
    pub fn get_flags(&self) -> u32 {
        self.data.mem_get_primitive::<u32>(28)
    }
    pub fn get_lladdr(&self) -> &[u8] {
        &self.data.as_mem()[32..32 + 6]
    }
    pub fn get_linkinfo(&self) -> () {}
}
impl<Buf: MemRead> MsgId for Link<Buf> {
    const MSGID: i32 = 10;
}
#[derive(Debug)]
pub struct Route4<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> Binder<Buf> for Route4<Buf> {
    fn bind_view(data: MemOffset<Buf>) -> Result<Self, BindError> {
        if data.mem_size() < 32 {
            return Err(BindError::new_size(32));
        }
        Ok(Self { data })
    }

    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        Self { data }
    }
}

impl<Buf: MemRead + Copy> Route4<Buf> {
    pub fn get_action(&self) -> Action {
        self.data.mem_get_primitive::<Action>(0)
    }
    pub fn get_table(&self) -> u32 {
        self.data.mem_get_primitive::<u32>(1)
    }
    pub fn get_type_(&self) -> RType {
        self.data.mem_get_primitive::<RType>(5)
    }
    pub fn get_oif(&self) -> Result<&'_ str, StringBindError> {
        tll::bind::byte_str(&self.data, 6, 16)
    }
    pub fn get_dst_mask(&self) -> u8 {
        self.data.mem_get_primitive::<u8>(22)
    }
    pub fn get_dst(&self) -> u32 {
        self.data.mem_get_primitive::<u32>(23)
    }
    pub fn get_src_mask(&self) -> u8 {
        self.data.mem_get_primitive::<u8>(27)
    }
    pub fn get_src(&self) -> u32 {
        self.data.mem_get_primitive::<u32>(28)
    }
}
impl<Buf: MemRead> MsgId for Route4<Buf> {
    const MSGID: i32 = 20;
}
#[derive(Debug)]
pub struct Route6<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> Binder<Buf> for Route6<Buf> {
    fn bind_view(data: MemOffset<Buf>) -> Result<Self, BindError> {
        if data.mem_size() < 56 {
            return Err(BindError::new_size(56));
        }
        Ok(Self { data })
    }

    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        Self { data }
    }
}

impl<Buf: MemRead + Copy> Route6<Buf> {
    pub fn get_action(&self) -> Action {
        self.data.mem_get_primitive::<Action>(0)
    }
    pub fn get_table(&self) -> u32 {
        self.data.mem_get_primitive::<u32>(1)
    }
    pub fn get_type_(&self) -> RType {
        self.data.mem_get_primitive::<RType>(5)
    }
    pub fn get_oif(&self) -> Result<&'_ str, StringBindError> {
        tll::bind::byte_str(&self.data, 6, 16)
    }
    pub fn get_dst_mask(&self) -> u8 {
        self.data.mem_get_primitive::<u8>(22)
    }
    pub fn get_dst(&self) -> &[u8] {
        &self.data.as_mem()[23..23 + 16]
    }
    pub fn get_src_mask(&self) -> u8 {
        self.data.mem_get_primitive::<u8>(39)
    }
    pub fn get_src(&self) -> &[u8] {
        &self.data.as_mem()[40..40 + 16]
    }
}
impl<Buf: MemRead> MsgId for Route6<Buf> {
    const MSGID: i32 = 30;
}
#[derive(Debug)]
pub struct Addr<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> Binder<Buf> for Addr<Buf> {
    fn bind_view(data: MemOffset<Buf>) -> Result<Self, BindError> {
        if data.mem_size() < 39 {
            return Err(BindError::new_size(39));
        }
        // Union
        Ok(Self { data })
    }

    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        Self { data }
    }
}

impl<Buf: MemRead + Copy> Addr<Buf> {
    pub fn get_action(&self) -> Action {
        self.data.mem_get_primitive::<Action>(0)
    }
    pub fn get_index(&self) -> i32 {
        self.data.mem_get_primitive::<i32>(1)
    }
    pub fn get_name(&self) -> Result<&'_ str, StringBindError> {
        tll::bind::byte_str(&self.data, 5, 16)
    }
    pub fn get_prefix(&self) -> u8 {
        self.data.mem_get_primitive::<u8>(21)
    }
    pub fn get_addr(&self) -> () {}
}
impl<Buf: MemRead> MsgId for Addr<Buf> {
    const MSGID: i32 = 40;
}
#[derive(Debug)]
pub struct Neigh<Buf: MemRead> {
    data: MemOffset<Buf>,
}
impl<Buf: MemRead + Copy> Binder<Buf> for Neigh<Buf> {
    fn bind_view(data: MemOffset<Buf>) -> Result<Self, BindError> {
        if data.mem_size() < 46 {
            return Err(BindError::new_size(46));
        }
        // Union
        Ok(Self { data })
    }

    fn bind_unchecked(data: MemOffset<Buf>) -> Self {
        Self { data }
    }
}

impl<Buf: MemRead + Copy> Neigh<Buf> {
    pub fn get_action(&self) -> Action {
        self.data.mem_get_primitive::<Action>(0)
    }
    pub fn get_index(&self) -> i32 {
        self.data.mem_get_primitive::<i32>(1)
    }
    pub fn get_name(&self) -> Result<&'_ str, StringBindError> {
        tll::bind::byte_str(&self.data, 5, 16)
    }
    pub fn get_state(&self) -> u16 {
        self.data.mem_get_primitive::<u16>(21)
    }
    pub fn get_addr(&self) -> () {}
    pub fn get_lladdr(&self) -> &[u8] {
        &self.data.as_mem()[40..40 + 6]
    }
}
impl<Buf: MemRead> MsgId for Neigh<Buf> {
    const MSGID: i32 = 50;
}
