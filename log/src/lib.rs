// http://lv2plug.in/ns/ext/log

#[macro_use]
extern crate lazy_static;
extern crate urid;

use std::os::raw;
use std::ffi::CString;


pub type LogHandle = *const raw::c_void;

// TODO: Do this better
lazy_static! {
    pub static ref URI: String = "http://lv2plug.in/ns/ext/log".to_string();
    pub static ref URI_PREFIX: String = ((*URI).to_string() + "#");
    pub static ref URI_log: String = ((*URI_PREFIX).to_string() + "log");
}

#[repr(C)]
struct LogFeature {
    pub handle: LogHandle,
    // TODO: variadic list
    pub printf: extern "C" fn(handle: LogHandle, type_id: urid::URID, *const raw::c_char, ...) -> i32,
    pub vprintf: extern "C" fn(handle: LogHandle, type_id: urid::URID, *const raw::c_char) -> i32,
}

pub trait Log {
    
}
