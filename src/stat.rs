//#![feature(const_generics)]

use tll_sys::stat::*;

use std::os::raw::c_char;
use std::ffi::CStr;
use std::str;
use std::cmp::{min, max};
use std::slice::from_raw_parts_mut;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Sum = TLL_STAT_SUM,
    Min = TLL_STAT_MIN,
    Max = TLL_STAT_MAX,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Unit {
    Unknown = TLL_STAT_UNIT_UNKNOWN,
    Bytes = TLL_STAT_UNIT_BYTES,
    Ns = TLL_STAT_UNIT_NS,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Field (tll_stat_field_t);

impl Field {
    pub fn new(name: &str, type_: Type) -> Field { Field::new_unit(name, type_, Unit::Unknown) }
    pub fn new_unit(name: &str, type_: Type, unit: Unit) -> Field
    {
        let t : u8 = ((type_ as u8) << 4) | (unit as u8 & 0xf);
        let mut n = [0 as u8; 7];
        let size = min(7, name.len());
        n[ .. size].clone_from_slice(&name.as_bytes()[ .. size]);
        let v = unsafe { tll_stat_value_t { value: tll_stat_default_int(type_ as tll_stat_type_t) }};
        Field(tll_stat_field_t { type_: t, name: n, value: v })
    }

    pub fn type_(&self) -> tll_stat_type_t { (self.0.type_ >> 4) & 0xf }
    pub fn unit(&self) -> tll_stat_unit_t { self.0.type_  & 0xf }
    pub fn name<'a>(&'a self) -> &'a str
    {
        let s = self.0.name.iter().position(|&x| x == 0);
        unsafe { str::from_utf8_unchecked(&self.0.name[0 .. s.unwrap_or(7)]) }
    }

    pub fn update(&mut self, value: i64)
    {
        match self.type_() {
            TLL_STAT_SUM => unsafe { self.0.value.value += value },
            TLL_STAT_MIN => unsafe { self.0.value.value = min(self.0.value.value, value) },
            TLL_STAT_MAX => unsafe { self.0.value.value = max(self.0.value.value, value) },
            _ => (),
        }
    }

    pub fn reset(&mut self)
    {
        unsafe { tll_stat_field_reset(&mut self.0) }
    }
}

//#[repr(C)]
//#[derive(Debug, Clone, Copy)]
//pub struct FieldT<N, const T : Type, const U : Unit>( Field );

#[derive(Debug, Clone, Copy)]
pub struct Page ( * mut tll_stat_page_t );

impl Page {
    pub fn fields<'a>(&'a self) -> &'a [Field]
    {
        if self.0.is_null() { return &[]; }

        let fields = unsafe { (*self.0).fields };
        if fields.is_null() { return &[]; }

        unsafe { from_raw_parts_mut(fields as *mut Field, (*self.0).size) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Block ( * mut tll_stat_block_t );

impl Block {
    pub fn acquire(&self) -> Option<Page>
    {
        let p = unsafe { tll_stat_page_acquire(self.0) };
        if p.is_null() { None } else { Some(Page(p)) }
    }

    pub fn release(&self, page: &Page)
    {
        unsafe { tll_stat_page_release(self.0, page.0) }
    }

    pub fn swap(&self) -> Option<Page>
    {
        let p = unsafe { tll_stat_block_swap(self.0) };
        if p.is_null() { None } else { Some(Page(p)) }
    }

    pub fn name<'a>(&'a self) -> &'a str
    {
        let n = unsafe { (*self.0).name };
        if n.is_null() { "" } else {
            unsafe { CStr::from_ptr(n) }.to_str().unwrap()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Iter(* mut tll_stat_iter_t);

impl Iter {
    pub fn name<'a>(&'a self) -> &'a str
    {
        let n = unsafe { tll_stat_iter_name(self.0) };
        if n.is_null() { "" } else {
            unsafe { CStr::from_ptr(n) }.to_str().unwrap()
        }
    }

    pub fn swap(&self) -> Option<Page>
    {
        let p = unsafe { tll_stat_iter_swap(self.0) };
        if p.is_null() { None } else { Some(Page(p)) }
    }

    pub fn block(&self) -> Option<Block>
    {
        let p = unsafe { tll_stat_iter_block(self.0) };
        if p.is_null() { None } else { Some(Block(p)) }
    }

    pub fn empty(&self) -> bool
    {
        unsafe { tll_stat_iter_empty(self.0) != 0 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct List(* mut tll_stat_list_t);

impl List {
    pub fn begin(&self) -> Iter
    {
        let i = unsafe { tll_stat_list_begin(self.0) };
        Iter(i)
    }
}

pub struct BasePage<T> {
    page: tll_stat_page_t,
    data: T,
}

impl <T: Default> BasePage<T> {
    pub fn new() -> BasePage<T>
    {
        let mut b = unsafe { BasePage {
            page: std::mem::zeroed::<tll_stat_page_t>(),
            data: Default::default(),
        }};
        b.page.fields = &mut b.data as *mut T as *mut tll_stat_field_t;
        b.page.size = std::mem::size_of::<T>() / std::mem::size_of::<tll_stat_field_t>();
        b
    }
}

pub struct Base<T> {
    block: tll_stat_block_t,
    data: [BasePage<T>; 2],
    name: String,
}

impl<T> std::fmt::Debug for Base<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("stat::Base {{ name: {} }}", self.name))
    }
}

pub struct Reference<'a, T : Default> {
    data: &'a mut BasePage<T>,
    base: *mut tll_stat_block_t,
}

impl <'a, T: Default> Reference<'a, T> {
    pub fn get(&mut self) -> &mut T { &mut self.data.data }
}

impl <'a, T: Default> Drop for Reference<'a, T> {
    fn drop(&mut self)
    {
        unsafe { tll_stat_page_release(self.base, &mut self.data.page) }
    }
}

impl <T: Default> Base<T> {
    pub fn new(name: &str) -> Base<T>
    {
        let mut b = unsafe { Base {
            block: std::mem::zeroed::<tll_stat_block_t>(),
            data: [BasePage::<T>::new(), BasePage::<T>::new()],
            name: name.to_string(),
        }};
        b.block.name = b.name.as_bytes().as_ptr() as *const c_char;
        b.block.active = &mut b.data[0].page;
        b.block.inactive = &mut b.data[1].page;
        b.block.lock = b.block.active;
        b
    }

    pub fn as_ptr(&mut self) -> * mut tll_stat_block_t { &mut self.block }

    pub fn acquire(&mut self) -> Option<Reference<T>>
    {
        let p = unsafe { tll_stat_page_acquire(&mut self.block) };
        if p.is_null() { return None }
        Some(Reference {data: &mut self.data[if p == &mut self.data[0].page { 0 } else { 1 }], base: &mut self.block})
    }

    pub fn release(&mut self, page: &T)
    {
        let p = &mut self.data[if page as *const T == &self.data[0].data as *const T { 0 } else { 1 }];
        unsafe { tll_stat_page_release(&mut self.block, &mut p.page) }
    }

    pub fn block(&mut self) -> Block { Block(&mut self.block) }
}
