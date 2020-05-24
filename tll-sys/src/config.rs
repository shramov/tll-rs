#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

//mod config {

use std::os::raw::c_int;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::option::Option;

#[ repr ( C ) ]
#[ derive ( Debug , Copy , Clone ) ]
pub struct tll_config_t { _unused : [ u8 ; 0 ] , }

pub type tll_config_callback_t = Option < unsafe extern "C" fn ( key : * const c_char , klen : c_int , value : * const tll_config_t , data : * mut c_void ) -> c_int > ;
pub type tll_config_value_callback_t = Option < unsafe extern "C" fn ( len : * mut c_int , data : * mut c_void ) -> * mut c_char > ;
pub type tll_config_load_t = Option < unsafe extern "C" fn ( path : * const c_char , plen : c_int , data : * mut c_void ) -> * mut tll_config_t > ;

extern "C" {
    pub fn tll_config_has ( cfg : * const tll_config_t , path : * const c_char , plen : c_int ) -> c_int ; 
    pub fn tll_config_del ( cfg : * mut tll_config_t , path : * const c_char , plen : c_int , recursive : c_int ) -> c_int ; 
    pub fn tll_config_set ( cfg : * mut tll_config_t , path : * const c_char , plen : c_int , value : * const c_char , vlen : c_int ) -> c_int ; 
    pub fn tll_config_set_config ( cfg : * mut tll_config_t , path : * const c_char , plen : c_int , arg2 : * mut tll_config_t ) -> c_int ; 
    pub fn tll_config_set_callback ( cfg : * mut tll_config_t , path : * const c_char , plen : c_int , cb : tll_config_value_callback_t , user : * mut c_void ) -> c_int ; 
    pub fn tll_config_merge ( cfg : * mut tll_config_t , src : * mut tll_config_t ) -> c_int ; 
    pub fn tll_config_get ( cfg : * const tll_config_t , path : * const c_char , plen : c_int , value : * mut c_char , vlen : * mut c_int ) -> c_int ; 
    pub fn tll_config_get_copy ( cfg : * const tll_config_t , path : * const c_char , plen : c_int , vlen : * mut c_int ) -> * mut c_char ; 
    pub fn tll_config_value_free ( value : * mut c_char ); 
    pub fn tll_config_value_dup ( value : * const c_char , len : c_int ) -> * mut c_char ; 
    pub fn tll_config_list ( cfg : * const tll_config_t , cb : tll_config_callback_t , data : * mut c_void ) -> c_int ; 
    pub fn tll_config_browse ( cfg : * const tll_config_t , mask : * const c_char , mlen : c_int , cb : tll_config_callback_t , data : * mut c_void ) -> c_int ; 
    pub fn tll_config_value ( cfg : * const tll_config_t ) -> c_int ;

    pub fn tll_config_load_register ( prefix : * const c_char , plen : c_int , cb : tll_config_load_t , data : * mut c_void ) -> c_int ; 
    pub fn tll_config_load_unregister ( prefix : * const c_char , plen : c_int , cb : tll_config_load_t , data : * mut c_void ) -> c_int ; 

    pub fn tll_config_new ( ) -> * mut tll_config_t ; 
    pub fn tll_config_load ( path : * const c_char , plen : c_int ) -> * mut tll_config_t ; 
    pub fn tll_config_sub ( cfg : * mut tll_config_t , path : * const c_char , plen : c_int ) -> * mut tll_config_t ; 
    pub fn tll_config_ref ( cfg : * const tll_config_t ); 
    pub fn tll_config_unref ( cfg : * const tll_config_t ); 
}
//}
