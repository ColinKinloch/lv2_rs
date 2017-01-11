use std::os::raw;
use std::slice;
use std::path::Path;
use std::mem;
use lv2;

use rand;
use std::f32;

#[repr(C)]
pub struct Oscillator {
    frequency: lv2::InputPort<lv2::ControlPort>,
    magnitude: lv2::InputPort<lv2::ControlPort>,
    saw: lv2::OutputPort<lv2::AudioPort>,
    sine: lv2::OutputPort<lv2::AudioPort>,
    square: lv2::OutputPort<lv2::AudioPort>,
    triangle: lv2::OutputPort<lv2::AudioPort>,
    white: lv2::OutputPort<lv2::AudioPort>,
    accumulator: f64,
    sample_rate: f64,
    rand: f32,
}

impl lv2::Plugin for Oscillator {
    fn new() -> Oscillator {
        unsafe { mem::zeroed::<Oscillator>() }
        /*Oscillator {
            frequency: lv2::InputPort(lv2::ControlPort(0.)),
            magnitude: lv2::InputPort(lv2::ControlPort(0.)),
            saw: lv2::OutputPort(lv2::AudioPort(0.)),
            sine: lv2::OutputPort(lv2::AudioPort(0.)),
            square: lv2::OutputPort(lv2::AudioPort(0.)),
            triangle: lv2::OutputPort(lv2::AudioPort(0.)),
            white: lv2::OutputPort(lv2::AudioPort(0.)),
            accumulator: 0.,
            sample_rate: 0.,
            rand: 0.,
        }*/
    }
    fn instantiate(&mut self, sample_rate: f64, bundle_path: &Path, feature: &[lv2::Feature]) {
        self.sample_rate = sample_rate;
        println!("{:?}", sample_rate);
        self.accumulator = 0.;
    }
    fn run(&mut self, sample_count: u32) {
        let frequency = unsafe { (*self.frequency.0).0 } as f64;
        let magnitude = unsafe { (*self.magnitude.0).0 } as f64;
        let saw = self.saw.as_slice_mut(sample_count);
        let sine = self.sine.as_slice_mut(sample_count);
        let square = self.square.as_slice_mut(sample_count);
        let triangle = self.triangle.as_slice_mut(sample_count);
        let white = self.white.as_slice_mut(sample_count);
        
        let d = (frequency * 10.0_f64.powf(magnitude)) / self.sample_rate;
        
        //TODO: Timing
        for ((((saw, sine), square), triangle), white) in
            saw.iter_mut().zip(sine.iter_mut()).zip(square.iter_mut()).zip(triangle.iter_mut()).zip(white.iter_mut()) {
            self.accumulator += 2. * d;
            //TODO: does this work?
            if self.accumulator > 1. {
                self.accumulator -= 2.;
                self.rand = rand::random::<rand::Closed01<f32>>().0;
            }
            let a = self.accumulator as f32;
            (*saw).0 = a;
            (*sine).0 = (a * f32::consts::PI).sin();
            (*square).0 = if a > 0. { 1. } else { -1. };
            (*triangle).0 = (a * 2.).abs() - 1.;
            (*white).0 = self.rand;
        }
        
    }
}

impl lv2::Connectable for Oscillator {
    fn connect_port(&mut self, port: u32, data: *mut raw::c_void) {
        match port {
            0 => self.frequency = lv2::InputPort::<lv2::ControlPort>::from(data),
            1 => self.magnitude = lv2::InputPort::<lv2::ControlPort>::from(data),
            2 => self.saw = lv2::OutputPort::<lv2::AudioPort>::from(data),
            3 => self.sine = lv2::OutputPort::<lv2::AudioPort>::from(data),
            4 => self.square = lv2::OutputPort::<lv2::AudioPort>::from(data),
            5 => self.triangle = lv2::OutputPort::<lv2::AudioPort>::from(data),
            6 => self.white = lv2::OutputPort::<lv2::AudioPort>::from(data),
            _ => panic!("Not a valid PortIndex: {}", port),
        }
    }
}
