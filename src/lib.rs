#![feature(try_from)]

extern crate rand;
extern crate heapsize;

mod vec;
mod mem;

use vec::*;
use mem::TotalMemory;
use heapsize::HeapSizeOf;

use std::convert::TryFrom;

/// A Gapush `Program` is a state machine which tracks the memory consumption of an arbitrary program and executes it
/// without going over a specified limit amount of memory (`max_memory`).
pub struct Program<Ins, InsHandler, IntHandler, FloatHandler> {
    /// The limit of how much memory is allowed to be used.
    pub max_memory: usize,
    /// Instructions are popped from this stack to be executed and this stack is not directly accessed.
    pub execute_stack: TrackedVec<Ins>,
    /// This is a stack for handling instructions as data.
    pub ins_stack: TrackedVec<Ins>,
    /// This is a stack for integers.
    pub int_stack: TrackedVec<i64>,
    /// This is a stack for floats.
    pub float_stack: TrackedVec<f64>,
    /// This is a stack for instruction vectors.
    pub ins_vec_stack: TrackedVec<TrackedVec<Ins>>,
    /// This is a stack for integer vectors.
    pub int_vec_stack: TrackedVec<TrackedVec<i64>>,
    /// This is a stack for float vectors.
    pub float_vec_stack: TrackedVec<TrackedVec<f64>>,
    
    /// This is called to produce an integer when one wasn't available.
    pub ins_handler: InsHandler,
    /// This is called to produce an integer when one wasn't available.
    pub int_handler: IntHandler,
    /// This is called to produce a float when one wasn't available.
    pub float_handler: FloatHandler,
}

impl<I, IH, IntH, FloatH> Program<I, IH, IntH, FloatH>
    where I: HeapSizeOf
{
    pub fn new(max_memory: usize, ins_handler: IH, int_handler: IntH, float_handler: FloatH) -> Self {
        Program {
            max_memory: max_memory,
            execute_stack: TrackedVec::new(),
            ins_stack: TrackedVec::new(),
            int_stack: TrackedVec::new(),
            float_stack: TrackedVec::new(),
            ins_vec_stack: TrackedVec::new(),
            int_vec_stack: TrackedVec::new(),
            float_vec_stack: TrackedVec::new(),
            ins_handler: ins_handler,
            int_handler: int_handler,
            float_handler: float_handler,
        }
    }
}

