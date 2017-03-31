#![feature(try_from)]

extern crate rand;
extern crate heapsize;

mod vec;
mod mem;

use vec::*;
use mem::TotalMemory;

use std::convert::TryFrom;

pub struct Program<Instruction> {
    /// Instructions are popped from this stack to be executed.
    pub execute_stack: TrackedVec<Instruction>,
    /// This is a stack for handling instructions as data.
    pub instruction_stack: TrackedVec<Instruction>,
}
