// http://lv2plug.in/ns/ext/atom

#![feature(untagged_unions)]

use std::mem;

#[repr(C)]
pub struct AtomPort<T>(pub T);

#[repr(C)]
pub struct Vector<T> {
    pub atom: Atom,
    pub child_size: u32,
    pub child_type: u32,
    pub elems: *const T,
}

//TODO: implement Sequence as AtomEvent Vector?
//TODO: Look at Glib and other loosely typed
#[repr(C)]
pub struct Sequence {
    pub atom: Atom,
    pub body: SequenceBody,
}

#[derive(Debug)]
#[repr(C)]
pub struct Atom {
    pub size: u32,
    pub type_id: u32,
    // Data
}

#[derive(Debug)]
#[repr(C)]
pub struct SequenceBody {
    pub unit: u32,
    pub pad: u32,
    // Events
}

#[repr(C)]
pub union EventsTime {
    frames: i64,
    beats: f32,
}

#[repr(C)]
pub struct Events {
    pub time: EventsTime,
    pub body: Atom,
}

impl Sequence {
    pub fn iter(&self) -> SequenceIter {
        unsafe {
            let mut ptr: *const Sequence = self;
            SequenceIter {
                current: ptr.offset(1) as *const u8,
                seq: self,
                end: (ptr as *const u8).offset(mem::size_of::<Atom>() as isize + self.atom.size as isize) as *const u8,
            }
        }
    }
}

impl Atom {
    pub fn get_padded_size(&self) -> u32 {
        (self.size + 7u32) & (!7u32)
    }
    pub fn get_content_ptr(&self) -> *const u8 {
        unsafe { (&*self as *const Atom as *const u8).offset(mem::size_of::<Atom>() as isize) }
    }
}

pub struct SequenceIter<'a> {
    seq: &'a Sequence,
    current: *const u8,
    end: *const u8,
}

impl<'a> Iterator for SequenceIter<'a> {
    type Item = &'a Events;
    
    fn next(&mut self) -> Option<&'a Events> {
        unsafe { 
            let current = self.current as *const Events;
            self.current = self.current.offset((((*current).body.get_padded_size()) as usize + mem::size_of::<Events>()) as isize);
            let cur = current as *const u8;
            if cur >= self.end {
                None
            } else {
                Some(&*current)
            }
        }
    }
}
