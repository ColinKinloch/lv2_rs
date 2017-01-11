use std::os::raw;
use std::slice;
use std::path::Path;
use std::mem;
use lv2;

#[repr(C)]
pub struct Scope {
    input: lv2::InputPort<lv2::AudioPort>,
}

#[repr(C)]
pub struct ScopeUI {
}

//lv2_plugin_connect_port_impl!(Amp, [gain, input, output]);

//TODO: macro, wrap, parse turtle, impl lv2 types, generate connect_port?
impl lv2::Plugin for Scope {
    fn new() -> Scope {
        unsafe { mem::zeroed::<Scope>() }
    }
    fn instantiate(&mut self, sample_rate: f64, bundle_path: &Path, feature: &[lv2::Feature]) {
    }
    fn run(&mut self, sample_count: u32) {
        let input = unsafe { slice::from_raw_parts(self.input.0, sample_count as usize) };
        
        for i in input.iter() {
            //println!("{:?}", i.0);
        }
    }
}

impl lv2::Connectable for Scope {
    fn connect_port(&mut self, port: u32, data: *mut raw::c_void) {
        match port {
            0 => self.input = lv2::InputPort::<lv2::AudioPort>::from(data),
            _ => panic!("Not a valid PortIndex: {}", port),
        }
    }
}
