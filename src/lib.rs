#![feature(try_from)]

extern crate rand;
extern crate heapsize;

mod vec;

use heapsize::HeapSizeOf;

use std::convert::TryFrom;
use std::mem;

pub trait TotalMemory {
    fn total_memory(&self) -> usize;
}

impl<T> TotalMemory for T where T: HeapSizeOf {
    fn total_memory(&self) -> usize {
        mem::size_of::<Self>() + self.heap_size_of_children()
    }
}
