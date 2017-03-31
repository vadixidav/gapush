#![feature(try_from)]

extern crate rand;
extern crate heapsize;

mod vec;
mod mem;

use vec::*;
use mem::TotalMemory;

use std::convert::TryFrom;

/// A Gapush `Program` is a state machine which tracks the memory consumption of an arbitrary program and executes it
/// without going over a specified limit amount of memory (`max_memory`).
pub struct Program<Instruction> {
    /// The limit of how much memory is allowed to be used.
    pub max_memory: usize,
    /// Instructions are popped from this stack to be executed and this stack is not directly accessed.
    pub execute_stack: TrackedVec<Instruction>,
    /// This is a stack for handling instructions as data.
    pub instruction_stack: TrackedVec<Instruction>,
}
