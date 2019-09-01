#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use std::os::raw::c_int;
use std::os::raw::c_char;

#[ repr ( C ) ]
#[ derive ( Debug , Copy , Clone ) ]
pub struct tll_logger_t { pub level : c_int , }

pub type tll_logger_level_t = u32; 
pub const TLL_LOGGER_TRACE : tll_logger_level_t = 0 ; 
pub const TLL_LOGGER_DEBUG : tll_logger_level_t = 1 ; 
pub const TLL_LOGGER_INFO : tll_logger_level_t = 2 ; 
pub const TLL_LOGGER_WARNING : tll_logger_level_t = 3 ; 
pub const TLL_LOGGER_ERROR : tll_logger_level_t = 4 ; 
pub const TLL_LOGGER_CRITICAL : tll_logger_level_t = 5 ; 

extern "C" {
    pub fn tll_logger_new ( name : * const c_char , len : c_int ) -> * mut tll_logger_t ; 
    pub fn tll_logger_free ( obj : * mut tll_logger_t );

    pub fn tll_logger_name ( obj : * const tll_logger_t ) -> * const c_char;

    pub fn tll_logger_set ( name : * const c_char , len : c_int, level : tll_logger_level_t, subtree : c_int ) -> c_int;
    pub fn tll_logger_log ( obj : * mut tll_logger_t, level : tll_logger_level_t, buf : * const c_char, size: usize ) -> c_int;
}
