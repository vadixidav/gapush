use {Instruction, Machine};
use heapsize::HeapSizeOf;
use rand;
use vec::*;
use std::mem;

/// Instructions which have implicit parameters and are encodable with a single integer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlainOp {
    /// integer: (a b -- (a + b))
    Addi64,
    /// integer: (a b -- (a - b))
    Subi64,
    /// integer: (a b -- (a * b))
    Muli64,
    /// integer: (a b -- (a / b))
    Divi64,
    /// integer: (a b -- (a << b))
    Rotli64,
    /// integer: (a b -- (a >> b))
    Rotri64,
    /// integer: (a b -- (a << b))
    Shftli64,
    /// integer: (a b -- (a >> b))
    Shftri64,
}

impl rand::Rand for PlainOp {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        // NOTE: Change whenever PlainOp is changed.
        // TODO: Switch to proc macros 1.1 framework when compiler plugin is developed.
        unsafe { mem::transmute(rng.gen_range(0u8, 8)) }
    }
}

impl HeapSizeOf for PlainOp {
    fn heap_size_of_children(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone)]
pub enum SimpleInstruction {
    PlainOp(PlainOp),
    BasicBlock(TrackedIter<SimpleInstruction>),
    Loop(TrackedCycleIter<SimpleInstruction>),
    If(TrackedIter<SimpleInstruction>, TrackedIter<SimpleInstruction>),
}

impl HeapSizeOf for SimpleInstruction {
    fn heap_size_of_children(&self) -> usize {
        use self::SimpleInstruction::*;
        match *self {
            PlainOp(_) => 0,
            BasicBlock(ref b) => b.heap_size_of_children(),
            Loop(ref l) => l.heap_size_of_children(),
            If(ref b0, ref b1) => b0.heap_size_of_children() + b1.heap_size_of_children(),
        }
    }
}

impl<IH, IntH, FloatH> Instruction<IH, IntH, FloatH> for SimpleInstruction
    where IH: FnMut() -> Self,
          IntH: FnMut() -> i64,
          FloatH: FnMut() -> f64
{
    fn operate(self, machine: &mut Machine<Self, IH, IntH, FloatH>) -> bool {
        use self::SimpleInstruction::*;
        use self::PlainOp::*;
        match self {
            PlainOp(Addi64) => {
                let b = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                let a = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a.wrapping_add(b)).is_ok()
            }
            PlainOp(Subi64) => {
                let b = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                let a = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a.wrapping_sub(b)).is_ok()
            }
            PlainOp(Muli64) => {
                let b = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                let a = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a.wrapping_mul(b)).is_ok()
            }
            PlainOp(Divi64) => {
                let b = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                let a = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                machine.state
                    .push_int(a.checked_div(b).unwrap_or_else(&mut machine.int_handler))
                    .is_ok()
            }
            PlainOp(Rotli64) => {
                let b = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                let a = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a.rotate_left(b.abs() as u32)).is_ok()
            }
            PlainOp(Rotri64) => {
                let b = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                let a = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a.rotate_right(b.abs() as u32)).is_ok()
            }
            PlainOp(Shftli64) => {
                let b = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                let a = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a << (b.abs() as u32)).is_ok()
            }
            PlainOp(Shftri64) => {
                let b = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                let a = machine.state.pop_int().unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a >> (b.abs() as u32)).is_ok()
            }
            BasicBlock(mut b) => {
                if let Some(i) = b.next() {
                    machine.state.push_exe(BasicBlock(b)).is_err() ||
                    machine.state.push_exe(i).is_err()
                } else {
                    false
                }
            }
            Loop(mut l) => {
                if let Some(i) = l.next() {
                    machine.state.push_exe(Loop(l)).is_err() || machine.state.push_exe(i).is_err()
                } else {
                    false
                }
            }
            If(b0, b1) => {
                let decider = machine.state.pop_bool().unwrap_or(false);
                machine.state.push_exe(BasicBlock(if decider { b0 } else { b1 })).is_ok()
            }
        }
    }
}

