// http://lv2plug.in/ns/ext/urid

use std::mem;
use std::os::raw;
use std::ffi::CStr;

pub static URI: &'static str = "http://lv2plug.in/ns/ext/urid";

pub type URID = u32;

pub type URIDMapHandle = *const raw::c_void;
pub type URIDUnmapHandle = *const raw::c_void;

#[repr(C)]
pub struct URIDMapDescriptor {
    pub handle: URIDMapHandle,
    pub map: extern "C" fn(handle: URIDMapHandle, uri: *const raw::c_char) -> URID,
}
#[repr(C)]
pub struct URIDUnmapDescriptor {
    pub handle: URIDUnmapHandle,
    pub unmap: extern "C" fn(handle: URIDUnmapHandle, uri: URID) -> *const raw::c_char,
}

pub trait URIDMap {
    fn map(&mut self, uri: &str) -> URID;
}

pub trait URIDMapRaw {
    extern "C" fn mapp(handle: URIDMapHandle, uri: *const raw::c_char) -> URID;
    fn get_urid_descriptor(&mut self) -> URIDMapDescriptor;
}

/*impl<T> URIDMapRaw for T
    where T: URIDMap {
    extern "C" fn mapp(handle: URIDUnmapHandle, uri: *const raw::c_char) -> URID {
        let plugin: &mut T = unsafe {mem::transmute(handle)};
        let uri = unsafe { CStr::from_ptr(uri) }.to_str().unwrap();
        plugin.map(uri)
    }
    fn get_urid_descriptor(&mut self) -> URIDMapDescriptor {
        URIDMapDescriptor {
            handle: &*self as *const raw::c_void,
            map: Self::mapp,
        }
    }
}*/
