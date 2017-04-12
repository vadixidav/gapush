extern crate rand;
extern crate heapsize;

mod vec;
mod mem;
mod state;
pub mod simple;

use state::*;

use mem::TotalMemory;
use heapsize::HeapSizeOf;

/// A Gapush `Machine` is a state machine which tracks the memory consumption of an arbitrary program and executes it
/// without going over a specified limit amount of memory (`max_memory`).
pub struct Machine<Ins, InsHandler, IntHandler, FloatHandler> {
    /// The internal state which instructions operate on.
    state: State<Ins>,
    
    /// This is called to produce an instruction when one wasn't available.
    ins_handler: InsHandler,
    /// This is called to produce an integer when one wasn't available.
    int_handler: IntHandler,
    /// This is called to produce a float when one wasn't available.
    float_handler: FloatHandler,
}

impl<I, IH, IntH, FloatH> Machine<I, IH, IntH, FloatH>
    where I: HeapSizeOf, IH: FnMut() -> I, IntH: FnMut() -> i64, FloatH: FnMut() -> f64
{
    pub fn new(max_size: usize, ins_handler: IH, int_handler: IntH, float_handler: FloatH) -> Self {
        Machine {
            state: State::new(max_size),
            ins_handler: ins_handler,
            int_handler: int_handler,
            float_handler: float_handler,
        }
    }

    /// Run a cycle of the machine unconditionally, executing an instruction produced by the instruction handler if
    /// necessary, and return whether or not the instruction executed was successful.
    pub fn cycle(&mut self) -> bool
        where I: Instruction<IH, IntH, FloatH>
    {
        self.state.pop_exe().unwrap_or_else(&mut self.ins_handler).operate(self)
    }
}

/// An instruction which can be executed on a `Machine`.
pub trait Instruction<IH, IntH, FloatH>: Sized {
    /// `operate` returns a boolean value which indicates the success of the operation.
    fn operate(self, &mut Machine<Self, IH, IntH, FloatH>) -> bool;
}

