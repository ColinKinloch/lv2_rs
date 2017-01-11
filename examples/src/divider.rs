use std::os::raw;
use std::slice;
use std::path::Path;
use std::mem;
use lv2;

use std::u16;

static CAP: u16 = 2 * 2 * 3 * 5 * 7;

#[repr(C)]
pub struct Divider {
    input: lv2::InputPort<lv2::AudioPort>,
    second: lv2::OutputPort<lv2::AudioPort>,
    third: lv2::OutputPort<lv2::AudioPort>,
    fifth: lv2::OutputPort<lv2::AudioPort>,
    seventh: lv2::OutputPort<lv2::AudioPort>,
    count: u16,
    prev: f32,
}

impl lv2::Plugin for Divider {
    fn new() -> Divider {
        unsafe { mem::zeroed::<Divider>() }
        /*Divider {
            input: lv2::InputPort(lv2::AudioPort(0.)),
            second: lv2::OutputPort(lv2::AudioPort(0.)),
            third: lv2::OutputPort(lv2::AudioPort(0.)),
            fifth: lv2::OutputPort(lv2::AudioPort(0.)),
            seventh: lv2::OutputPort(lv2::AudioPort(0.)),
            count: 0,
            prev: 0.,
        }*/
    }
    fn instantiate(&mut self, sample_rate: f64, bundle_path: &Path, feature: &[lv2::Feature]) {
        self.count = 0;
    }
    fn run(&mut self, sample_count: u32) {
        let input = self.input.as_slice(sample_count);
        let second = self.second.as_slice_mut(sample_count);
        let third = self.third.as_slice_mut(sample_count);
        let fifth = self.fifth.as_slice_mut(sample_count);
        let seventh = self.seventh.as_slice_mut(sample_count);
        
        for ((((i, o2), o3), o5), o7) in
          input.iter()
            .zip(second.iter_mut())
            .zip(third.iter_mut())
            .zip(fifth.iter_mut())
            .zip(seventh.iter_mut()) {
            if (i.0 - self.prev).abs() > 0.5 {
                self.count = (self.count + 1) % CAP;
            }
            
            self.prev = i.0;
            
            (*o2).0 = ((self.count as f32 % 4.) / 2.) as u32 as f32 * 2. - 1.;
            (*o3).0 = ((self.count as f32 % 6.) / 3.) as u32 as f32 * 2. - 1.;
            (*o5).0 = ((self.count as f32 % 10.) / 5.) as u32 as f32 * 2. - 1.;
            (*o7).0 = ((self.count as f32 % 14.) / 7.) as u32 as f32 * 2. - 1.;
        }
        
    }
}

impl lv2::Connectable for Divider {
    fn connect_port(&mut self, port: u32, data: *mut raw::c_void) {
        match port {
            0 => self.input = lv2::InputPort::<lv2::AudioPort>::from(data),
            1 => self.second = lv2::OutputPort::<lv2::AudioPort>::from(data),
            2 => self.third = lv2::OutputPort::<lv2::AudioPort>::from(data),
            3 => self.fifth = lv2::OutputPort::<lv2::AudioPort>::from(data),
            4 => self.seventh = lv2::OutputPort::<lv2::AudioPort>::from(data),
            _ => panic!("Not a valid PortIndex: {}", port),
        }
    }
}
