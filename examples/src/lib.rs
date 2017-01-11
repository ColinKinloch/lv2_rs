extern crate rand;

#[macro_use]
extern crate lv2core as lv2;
extern crate atom as atom;
extern crate urid as urid;
extern crate log as log;
extern crate midi as midi;

mod amp;
mod osc;
mod quant;
mod midigate;
mod scope;
mod divider;

lv2_descriptor! [
    b"http://colin.kinlo.ch/lv2/rust/amp\0" => amp::Amplifier,
    b"http://colin.kinlo.ch/lv2/rust/osc\0" => osc::Oscillator,
    b"http://colin.kinlo.ch/lv2/rust/quant\0" => quant::Quantiser,
    b"http://colin.kinlo.ch/lv2/rust/midigate\0" => midigate::MidiGate,
    b"http://colin.kinlo.ch/lv2/rust/scope\0" => scope::Scope,
    b"http://colin.kinlo.ch/lv2/rust/divider\0" => divider::Divider
];

