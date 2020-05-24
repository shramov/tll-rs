#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use std::fmt;
use std::os::raw::{c_int, c_char};

pub type tll_stat_type_t = u8; 
pub const TLL_STAT_SUM : tll_stat_type_t = 0; 
pub const TLL_STAT_MIN : tll_stat_type_t = 1; 
pub const TLL_STAT_MAX : tll_stat_type_t = 2; 

pub type tll_stat_unit_t = u8; 
pub const TLL_STAT_UNIT_UNKNOWN : tll_stat_unit_t = 0; 
pub const TLL_STAT_UNIT_BYTES : tll_stat_unit_t = 1; 
pub const TLL_STAT_UNIT_NS : tll_stat_unit_t = 2; 

#[ repr ( C ) ]
#[ derive ( Copy, Clone ) ]
pub union tll_stat_value_t {
    pub value : i64,
    pub fvalue : f64,
}

impl fmt::Debug for tll_stat_value_t {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { write!(f, "tll_stat_value_t {}", self.value) }
    }
}

#[ repr ( C ) ]
#[ derive ( Debug, Copy, Clone ) ]
pub struct tll_stat_field_t {
    pub type_ : u8,
    pub name : [u8; 7],
    pub value : tll_stat_value_t,
}

#[ repr ( C ) ]
#[ derive ( Debug, Copy, Clone ) ]
pub struct tll_stat_page_t {
    pub fields : * mut tll_stat_field_t,
    pub size : usize,
}

#[ repr ( C ) ]
#[ derive ( Debug, Copy, Clone ) ]
pub struct tll_stat_block_t {
    pub lock : * mut tll_stat_page_t,
    pub active : * mut tll_stat_page_t,
    pub inactive : * mut tll_stat_page_t,
    pub name : * const c_char,
}

#[ derive ( Debug ) ]
pub enum tll_stat_list_t {}

#[ derive ( Debug ) ]
pub enum tll_stat_iter_t {}

extern "C" {
    pub fn tll_stat_default_int( type_ : tll_stat_type_t ) -> i64;
    pub fn tll_stat_default_float( type_ : tll_stat_type_t ) -> f64;
    pub fn tll_stat_field_reset( obj : * mut tll_stat_field_t );

    pub fn tll_stat_page_acquire( obj : * mut tll_stat_block_t ) -> * mut tll_stat_page_t;
    pub fn tll_stat_page_release( obj : * mut tll_stat_block_t, page : * mut tll_stat_page_t );

    pub fn tll_stat_block_swap( obj : * mut tll_stat_block_t ) -> * mut tll_stat_page_t;

    pub fn tll_stat_list_new() -> * mut tll_stat_list_t ; 
    pub fn tll_stat_list_free( obj : * mut tll_stat_list_t );

    pub fn tll_stat_list_add( obj : * mut tll_stat_list_t, block : * mut tll_stat_block_t ) -> c_int;
    pub fn tll_stat_list_remove( obj : * mut tll_stat_list_t, block : * mut tll_stat_block_t ) -> c_int;

    pub fn tll_stat_list_begin( obj : * mut tll_stat_list_t ) -> * mut tll_stat_iter_t;
    pub fn tll_stat_iter_next( obj : * mut tll_stat_iter_t ) -> * mut tll_stat_iter_t;
    pub fn tll_stat_iter_empty( obj : * const tll_stat_iter_t ) -> c_int;
    pub fn tll_stat_iter_name( obj : * const tll_stat_iter_t ) -> * const c_char;
    pub fn tll_stat_iter_swap( obj : * mut tll_stat_iter_t ) -> * mut tll_stat_page_t;
    pub fn tll_stat_iter_block( obj : * mut tll_stat_iter_t ) -> * mut tll_stat_block_t;
}
