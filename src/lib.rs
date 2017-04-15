extern crate rand;
extern crate heapsize;
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod vec;
mod mem;
mod state;
pub mod simple;

use state::*;

use mem::TotalMemory;
use heapsize::HeapSizeOf;

/// A Gapush `Machine` is a state machine which tracks the memory consumption of an arbitrary program and executes it
/// without going over a specified limit amount of memory (`max_memory`).
#[derive(Debug, Clone)]
pub struct Machine<Ins, InsHandler, IntHandler, FloatHandler> {
    /// The internal state which instructions operate on.
    pub state: State<Ins>,
    
    /// This is called to produce an instruction when one wasn't available.
    pub ins_handler: InsHandler,
    /// This is called to produce an integer when one wasn't available.
    pub int_handler: IntHandler,
    /// This is called to produce a float when one wasn't available.
    pub float_handler: FloatHandler,
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
    pub fn cycle(&mut self) -> (Option<I>, bool)
        where I: Instruction<IH, IntH, FloatH>
    {
        self.state.pop_exe().unwrap_or_else(&mut self.ins_handler).operate(self)
    }

    /// Provide instruction, returning true if successful.
    pub fn provide(&mut self, ins: I) -> bool
        where I: Instruction<IH, IntH, FloatH>
    {
        self.state.push_exe(ins).is_ok()
    }

    /// Cycle up to a limit to produce an instruction.
    ///
    /// This also returns the number of cycles performed.
    pub fn cycle_until(&mut self, count: usize) -> (Option<I>, usize)
        where I: Instruction<IH, IntH, FloatH>
    {
        (0..count).map(|i| (self.cycle().0, i)).find(|&(ref ins, _)| ins.is_some()).unwrap_or((None, count))
    }

    /// Combines behavior of provide() and cycle_until().
    pub fn provide_and_cycle_until(&mut self, count: usize, ins: I) -> (Option<I>, usize)
        where I: Instruction<IH, IntH, FloatH>
    {
        if !self.provide(ins) {
            (None, 0)
        } else {
            (0..count).map(|i| (self.cycle().0, i)).find(|&(ref ins, _)| ins.is_some()).unwrap_or((None, count))
        }
    }
}

/// An instruction which can be executed on a `Machine`.
pub trait Instruction<IH, IntH, FloatH>: Sized {
    /// `operate` returns a boolean value which indicates the success of the operation.
    fn operate(self, &mut Machine<Self, IH, IntH, FloatH>) -> (Option<Self>, bool);
}

