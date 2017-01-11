use std::os::raw;
use std::slice;
use std::path::Path;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt::Display;
use std::mem;
use lv2;
use atom;
use urid;
use midi;

#[repr(C)]
pub struct MidiGateURIs {
    midi_MidiEvent: urid::URID,
}

#[repr(C)]
pub struct MidiGate {
    control: lv2::InputPort<atom::AtomPort<atom::Sequence>>,
    input: lv2::InputPort<lv2::AudioPort>,
    output: lv2::OutputPort<lv2::AudioPort>,
    map: *const urid::URIDMapDescriptor,
    uris: MidiGateURIs,
    active_note_count: u8,
    program: u8,
}

impl lv2::Plugin for MidiGate {
    fn new() -> MidiGate {
        unsafe { mem::zeroed::<MidiGate>() }
    }
    fn instantiate(&mut self, sample_rate: f64, bundle_path: &Path, features: &[lv2::Feature]) {
        for feature in features {
            let uri = unsafe { CStr::from_ptr(feature.uri) }.to_string_lossy();
            println!("{}", uri);
            if uri == urid::URI.to_string() + "#map" && !feature.data.is_null() {
                let map = feature.data as *const urid::URIDMapDescriptor;
                self.map = feature.data as *const urid::URIDMapDescriptor;
                self.uris.midi_MidiEvent = unsafe {
                    ((*map).map)((*map).handle,
                        CString::new(midi::URI.to_string() + "#MidiEvent").unwrap().as_ptr())
                };
                
                //feature.data
            }
        }
    }
    fn run(&mut self, sample_count: u32) {
        let control = unsafe { &*self.control.0 };
        let input = unsafe { slice::from_raw_parts(self.input.0, sample_count as usize) };
        let output = unsafe { slice::from_raw_parts_mut(self.output.0, sample_count as usize) };
        
        for (i, event) in control.0.iter().enumerate() {
            if self.uris.midi_MidiEvent == event.body.type_id {
                let (message, key, velocity) = unsafe {
                    let b = &*event as *const atom::Events as *const u32;
                    // println!("{}", *b.offset(3)); Event type id?
                    //println!("{}", *b.offset(5));
                    let message = b.offset(4);
                    let m = message as *const u8;
                    let m_type = m.offset(0) as *const midi::MessageType;
                    let vel = *m.offset(2) as f32 / 2.0_f32.powi(7);
                    (&*m_type, *m.offset(1), vel)
                };
                
                match *message {
                    midi::MessageType::NoteOn => self.active_note_count += key,
                    midi::MessageType::NoteOff => self.active_note_count -= key,
                    midi::MessageType::PgmChange => {
                        if key == 0 || key == 1 {
                            self.program = key;
                        }
                    },
                    _ => println!("{}: {:?}:{}:{}", i, message, key, velocity),
                }
            }
        }
        let active = if self.program == 0 {
            (self.active_note_count > 0)
        } else {
            (self.active_note_count == 0)
        };
        if active {
            for (o, i) in output.iter_mut().zip(input) {
                (*o).0 = i.0;
            }
        } else {
            for o in output.iter_mut() {
                (*o).0 = 0.;
            }
        }
    }
}

impl lv2::Connectable for MidiGate {
    fn connect_port(&mut self, port: u32, data: *mut raw::c_void) {
        match port {
            0 => self.control = lv2::InputPort::<atom::AtomPort<atom::Sequence>>::from(data),
            1 => self.input = lv2::InputPort::<lv2::AudioPort>::from(data),
            2 => self.output = lv2::OutputPort::<lv2::AudioPort>::from(data),
            _ => panic!("Not a valid PortIndex: {}", port),
        }
    }
}

impl urid::URIDMap for MidiGate {
    fn map(&mut self, uri: &str) -> urid::URID {
        println!("{}", uri);
        0
    }
}
