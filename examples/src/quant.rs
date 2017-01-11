use std::os::raw;
use std::slice;
use std::path::Path;
use std::mem;
use lv2;

use std::f32;

#[repr(C)]
pub struct Quantiser {
    step_size: lv2::InputPort<lv2::ControlPort>,
    input: lv2::InputPort<lv2::AudioPort>,
    output: lv2::OutputPort<lv2::AudioPort>,
}

impl lv2::Plugin for Quantiser {
    fn new() -> Quantiser {
        unsafe { mem::zeroed::<Quantiser>() }
        /*Quantiser {
            step_size: lv2::InputPort(lv2::ControlPort(0.)),
            input: lv2::InputPort(lv2::AudioPort(0.)),
            output: lv2::OutputPort(lv2::AudioPort(0.)),
        }*/
    }
    fn instantiate(&mut self, sample_rate: f64, bundle_path: &Path, feature: &[lv2::Feature]) {
    }
    fn run(&mut self, sample_count: u32) {
        let step_size = unsafe { (*self.step_size.0).0 };
        let input = unsafe { slice::from_raw_parts(self.input.0, sample_count as usize) };
        let output = unsafe { slice::from_raw_parts_mut(self.output.0, sample_count as usize) };
        
        for (o, i) in output.iter_mut().zip(input.iter()) {
            (*o).0 = step_size * (i.0 / step_size + 0.5).floor() ;
        }
        
    }
}

impl lv2::Connectable for Quantiser {
    fn connect_port(&mut self, port: u32, data: *mut raw::c_void) {
        match port {
            0 => self.step_size = lv2::InputPort::<lv2::ControlPort>::from(data),
            1 => self.input = lv2::InputPort::<lv2::AudioPort>::from(data),
            2 => self.output = lv2::OutputPort::<lv2::AudioPort>::from(data),
            _ => panic!("Not a valid PortIndex: {}", port),
        }
    }
}
