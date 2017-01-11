use std::os::raw;
use std::slice;
use std::path::Path;
use std::mem;
use lv2;

#[repr(C)]
pub struct Amplifier {
    gain: lv2::InputPort<lv2::ControlPort>,
    input: lv2::InputPort<lv2::AudioPort>,
    output: lv2::OutputPort<lv2::AudioPort>,
}

//lv2_plugin_connect_port_impl!(Amplifier, [gain, input, output]);

//TODO: macro, wrap, parse turtle, impl lv2 types, generate connect_port?
impl lv2::Plugin for Amplifier {
    fn new() -> Amplifier {
        unsafe { mem::zeroed::<Amplifier>() }
        /*Amplifier {
            gain: lv2::InputPort(lv2::ControlPort(0.)),
            input: lv2::InputPort(lv2::AudioPort(0.)),
            output: lv2::OutputPort(lv2::AudioPort(0.)),
        }*/
    }
    fn instantiate(&mut self, sample_rate: f64, bundle_path: &Path, feature: &[lv2::Feature]) {
    }
    fn run(&mut self, sample_count: u32) {
        let gain = unsafe { (*self.gain.0).0 };
        let input = unsafe { slice::from_raw_parts(self.input.0, sample_count as usize) };
        let output = unsafe { slice::from_raw_parts_mut(self.output.0, sample_count as usize) };
        
        let coef: f32 = if gain > -90.0 { (10.0 as f32).powf(gain * 0.05) } else { 0.0 };
        
        for (o, i) in output.iter_mut().zip(input) {
            (*o).0 = i.0 * coef;
        }
    }
}

impl lv2::Connectable for Amplifier {
    fn connect_port(&mut self, port: u32, data: *mut raw::c_void) {
        match port {
            0 => self.gain = lv2::InputPort::<lv2::ControlPort>::from(data),
            1 => self.input = lv2::InputPort::<lv2::AudioPort>::from(data),
            2 => self.output = lv2::OutputPort::<lv2::AudioPort>::from(data),
            _ => panic!("Not a valid PortIndex: {}", port),
        }
    }
}
