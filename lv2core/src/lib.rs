use std::os::raw;
use std::ops;
use std::mem;
use std::slice;
use std::ptr;
use std::ffi::CStr;
use std::ffi::CString;
use std::path::Path;


pub type URI = *const raw::c_char;
pub type Handle = *mut raw::c_void;

#[repr(C)]
pub struct Descriptor {
    pub uri: URI,
    pub instantiate: extern "C" fn(descriptor: *const Descriptor,
                                   sample_rate: f64,
                                   bundle_path: *const raw::c_char,
                                   features: *const (*const Feature))
                                   -> Handle,
    pub connect_port: extern "C" fn(handle: Handle, port: u32, data: *mut raw::c_void),
    pub activate: extern "C" fn(instance: Handle),
    pub run: extern "C" fn(instance: Handle, sample_count: u32),
    pub deactivate: extern "C" fn(instance: Handle),
    pub cleanup: extern "C" fn(instance: Handle),
    pub extension_data: extern "C" fn(uri: *const u8) -> (*const raw::c_void),
}

#[derive(Clone)]
#[repr(C)]
pub struct Feature {
    pub uri: URI,
    pub data: *mut raw::c_void,
}

pub trait Plugin {
    fn new() -> Self;
    fn instantiate(&mut self, sample_rate: f64, bundle_path: &Path, features: &[Feature]);
    fn run(&mut self, sample_count: u32);
}

pub trait Connectable {
    fn connect_port(&mut self, port: u32, data: *mut raw::c_void);
}

//macro_rules! plugin

#[repr(C)]
pub struct InputPort<T>(pub *const T);
#[repr(C)]
pub struct OutputPort<T>(pub *mut T);

impl<T> InputPort<T> {
    pub fn as_slice(&self, sample_count: u32) -> &[T] {
        unsafe { slice::from_raw_parts(self.0, sample_count as usize) }
    }
}

impl<T> OutputPort<T> {
    pub fn as_slice(&self, sample_count: u32) -> &[T] {
        unsafe { slice::from_raw_parts(self.0, sample_count as usize) }
    }
    pub fn as_slice_mut(&self, sample_count: u32) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.0, sample_count as usize) }
    }
}

#[repr(C)]
pub struct AudioPort(pub f32);
#[repr(C)]
pub struct ControlPort(pub f32);

/*impl<T> Into<T> for InputPort<T> {
    fn into(self) -> T {
        *self.0 as T
    }
}*/

impl<T> From<*mut raw::c_void> for InputPort<T> {
    fn from(data: *mut raw::c_void) -> Self {
        InputPort(data as *const T) as InputPort<T>
    }
}
impl<T> From<*mut raw::c_void> for OutputPort<T> {
    fn from(data: *mut raw::c_void) -> Self {
        OutputPort(data as *mut T) as OutputPort<T>
    }
}

/*#[macro_export]
macro_rules! lv2_plugin_connect_port_count_impl {
    ( @step $_idx: pat, ) => {};
    ( @step $idx: pat, [ $member:ident, $($tail: ident,)* ] ) => {
        $idx => self.$member = $crate::InputPort::<$crate::ControlPort>::from(data),
        lv2_plugin_connect_port_count_impl!(@step $idx + 1usize, [ $( $tail ),* ])
    };
    ( [ $( $port: ident ),* ] ) => {
        lv2_plugin_connect_port_count_impl!(@step 0usize, [ $( $port, ),* ])
    }
}*/

#[macro_export]
macro_rules! lv2_plugin_connect_port_impl {
    ( $plugin: ty, [ $( $member: ident ),* ] ) => {
        impl $plugin {
            $(
                fn get_$member(&mut self) -> $member {
                    self.$member
                }
            ),*
        }
        
        impl $crate::Connectable for $plugin {
            fn connect_port(&mut self, port: u32, data: *mut raw::c_void) {
            }
        }
    }
}

#[macro_export]
macro_rules! lv2_descriptor {
    ( $( $uri: expr => $plugin: ty ),* ) => {
        use $crate::RawPlugin;
        static mut MODULES: &'static [$crate::Descriptor] = &[
            $(
                $crate::Descriptor {
                    // HACK: arm require strings to be i8 rather than u8
                    uri: $uri as *const _ as *const _,
                    instantiate: <$plugin>::instantiate,
                    connect_port: <$plugin>::connect_port,
                    activate: <$plugin>::activate,
                    run: <$plugin>::run,
                    deactivate: <$plugin>::deactivate,
                    cleanup: <$plugin>::cleanup,
                    extension_data: <$plugin>::extension_data,
                },
            )*
        ];
        
        #[no_mangle]
        pub extern "C" fn lv2_descriptor(index: u32) -> *const $crate::Descriptor {
            use std::ptr;
            unsafe {
                match MODULES.get(index as usize) {
                    Some(d) => d,
                    None => ptr::null(),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! lv2_uri_prefix {
    ($e: expr, $p: expr) => {
        ($e.to_string() + "#".to_string() + $p.to_string())
    }
}

pub trait RawPlugin {
    extern "C" fn instantiate(descriptor: *const Descriptor,
                              sample_rate: f64,
                              bundle_path: *const raw::c_char,
                              features: *const (*const Feature))
                              -> *mut raw::c_void;
    extern "C" fn connect_port(handle: Handle, port: u32, data: *mut raw::c_void);
    extern "C" fn activate(handle: Handle);
    extern "C" fn run(handle: Handle, sample_count: u32);
    extern "C" fn deactivate(handle: Handle);
    extern "C" fn cleanup(handle: Handle);
    extern "C" fn extension_data(uri: *const u8) -> *const raw::c_void;
}

impl<T> RawPlugin for T
    where T: Plugin + Connectable {
    extern "C" fn instantiate(descriptor: *const Descriptor,
                              sample_rate: f64,
                              bundle_path: *const raw::c_char,
                              features: *const (*const Feature))
                              -> *mut raw::c_void {
        let mut f = Box::new( T::new() );
        let path_cstr = unsafe { CStr::from_ptr(bundle_path) };
        let bundle_path = Path::new(path_cstr.to_str().unwrap());
        let mut i = 0;
        let mut fu = unsafe { *features.offset(i) };
        while !fu.is_null() {
            i += 1;
            unsafe {
                fu = *features.offset(i);
//                println!("{:?}", CStr::from_ptr((*fu).uri));
            }
        }
        let fet_slice = unsafe { slice::from_raw_parts(features, i as usize)
          .iter().map(|v| {(*(*v)).clone()})}.collect::<Vec<_>>();
        f.instantiate(sample_rate, bundle_path, fet_slice.as_slice());
        Box::into_raw(f) as *mut _
    }
    extern "C" fn connect_port(handle: Handle, port: u32, data: *mut raw::c_void) {
        let plugin: &mut T = unsafe {mem::transmute(handle)};
        plugin.connect_port(port, data)
    }
    extern "C" fn activate(handle: Handle) {}
    extern "C" fn run(handle: Handle, sample_count: u32) {
        let plugin: &mut T = unsafe {mem::transmute(handle)};
        plugin.run(sample_count);
    }
    extern "C" fn deactivate(handle: Handle) {}
    extern "C" fn cleanup(handle: Handle) {
        unsafe { Box::from_raw(handle); }
    }
    extern "C" fn extension_data(uri: *const u8) -> *const raw::c_void {
        ptr::null()
    }
}
