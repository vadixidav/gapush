use {Instruction, Machine};
use heapsize::HeapSizeOf;
use rand;
use vec::*;
use std::mem;

/// Instructions which have implicit parameters and are encodable with a single integer.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PlainOp {
    // Integer operations
    /// integer: (a -- a++)
    Inci64,
    /// integer: (a -- a--)
    Deci64,
    /// integer: (a b -- (a + b))
    Addi64,
    /// integer: (a b -- (a - b))
    Subi64,
    /// integer: (a b -- (a * b))
    Muli64,
    /// integer: (a b -- (a / b))
    Divi64,
    /// integer: (a b -- (a % b))
    Remi64,
    /// integer: (a -- -a)
    Negi64,
    /// integer: (a -- |a|)
    Absi64,
    /// integer: (a -- |a|)
    Powi64,
    /// integer: (a b -- (a << b))
    Rotli64,
    /// integer: (a b -- (a >> b))
    Rotri64,
    /// integer: (a b -- (a << b))
    Shftli64,
    /// integer: (a b -- (a >> b))
    Shftri64,
    /// integer: (a b -- (a & b))
    Andi64,
    /// integer: (a b -- (a | b))
    Ori64,
    /// integer: (a b -- (a ^ b))
    Xori64,
    /// integer: (a -- ~a)
    Invi64,
    /// integer: (a b -- )
    /// bool: ( -- a < b)
    Lesi64,
    /// integer: (a b -- )
    /// bool: ( -- a > b)
    Grti64,
    /// integer: (a b -- )
    /// bool: ( -- a = b)
    Eqi64,
    /// integer: (a b -- )
    /// bool: ( -- a != b)
    Neqi64,

    // Floating point operations
    /// float: (a -- a++)
    Incf64,
    /// float: (a -- a--)
    Decf64,
    /// float: (a b -- (a + b))
    Addf64,
    /// float: (a b -- (a - b))
    Subf64,
    /// float: (a b -- (a * b))
    Mulf64,
    /// float: (a b -- (a / b))
    Divf64,
    /// float: (a b -- (a % b))
    Remf64,
    /// float: (a -- -a)
    Negf64,
    /// float: (a -- |a|)
    Absf64,
    /// float: (a -- a**b)
    /// int: (b -- )
    Powif64,
    /// float: (a b -- a**b)
    Powff64,
    /// float: (a b -- )
    /// bool: ( -- a < b)
    Lesf64,
    /// float: (a b -- )
    /// bool: ( -- a > b)
    Grtf64,
    /// float: (a b -- )
    /// bool: ( -- a = b)
    Eqf64,
    /// float: (a b -- )
    /// bool: ( -- a != b)
    Neqf64,

    // Boolean operations
    /// bool: (a b -- a && b)
    Andb,
    /// bool: (a b -- a || b)
    Orb,
    /// bool: (a b -- a == b)
    Eqb,
    /// bool: (a b -- a != b)
    Neqb,
    /// bool: (a -- !a)
    Notb,

    // Conversion operations
    /// int: (a -- )
    /// float: ( -- a)
    Itof,
    /// float: (a -- )
    /// int: ( -- a)
    Ftoi,

    // Stack manipulation
    /// int: (b -- )
    /// ins: (a b.. -- b.. a)
    Rotins,
    /// int: (a b.. b -- b.. a)
    Roti64,
    /// int: (b -- )
    /// float: (a b.. -- b.. a)
    Rotf64,
    /// int: (b -- )
    /// bool: (a b.. -- b.. a)
    Rotb,
    /// int: (b -- )
    /// ins vec: (a b.. -- b.. a)
    Rotinsv,
    /// int: (b -- )
    /// int vec: (a b.. -- b.. a)
    Roti64v,
    /// int: (b -- )
    /// float vec: (a b.. -- b.. a)
    Rotf64v,
    /// int: (b -- )
    /// ins: (a b.. -- a b.. a)
    Copyins,
    /// int: (a b.. b -- a b.. a)
    Copyi64,
    /// int: (b -- )
    /// float: (a b.. -- a b.. a)
    Copyf64,
    /// int: (b -- )
    /// bool: (a b.. -- a b.. a)
    Copyb,
    /// int: (b -- )
    /// ins vec: (a b.. -- a b.. a)
    Copyinsv,
    /// int: (b -- )
    /// int vec: (a b.. -- a b.. a)
    Copyi64v,
    /// int: (b -- )
    /// float vec: (a b.. -- a b.. a)
    Copyf64v,
}

impl rand::Rand for PlainOp {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        // NOTE: Change whenever PlainOp is changed.
        // TODO: Switch to proc macros 1.1 framework when compiler plugin is developed.
        unsafe { mem::transmute(rng.gen_range(0u8, 58)) }
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
            PlainOp(Inci64) => {
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a.wrapping_add(1)).is_ok()
            }
            PlainOp(Deci64) => {
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a.wrapping_sub(1)).is_ok()
            }
            PlainOp(Addi64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a.wrapping_add(b)).is_ok()
            }
            PlainOp(Subi64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a.wrapping_sub(b)).is_ok()
            }
            PlainOp(Muli64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a.wrapping_mul(b)).is_ok()
            }
            PlainOp(Divi64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .push_int(a.checked_div(b).unwrap_or_else(&mut machine.int_handler))
                    .is_ok()
            }
            PlainOp(Remi64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .push_int(a.checked_rem(b).unwrap_or_else(&mut machine.int_handler))
                    .is_ok()
            }
            PlainOp(Negi64) => {
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .push_int(a.checked_neg().unwrap_or_else(&mut machine.int_handler))
                    .is_ok()
            }
            PlainOp(Absi64) => {
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .push_int(a.checked_abs().unwrap_or_else(&mut machine.int_handler))
                    .is_ok()
            }
            PlainOp(Powi64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .push_int(a.pow((b.abs() & (0xFFFFFFFF)) as u32))
                    .is_ok()
            }
            PlainOp(Rotli64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .push_int(a.rotate_left((b & (0xFFFFFFFF)) as u32))
                    .is_ok()
            }
            PlainOp(Rotri64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .push_int(a.rotate_right((b & (0xFFFFFFFF)) as u32))
                    .is_ok()
            }
            PlainOp(Shftli64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .push_int(a.checked_shl((b & (0xFFFFFFFF)) as u32)
                                  .unwrap_or_else(&mut machine.int_handler))
                    .is_ok()
            }
            PlainOp(Shftri64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .push_int(a.checked_shr((b & (0xFFFFFFFF)) as u32)
                                  .unwrap_or_else(&mut machine.int_handler))
                    .is_ok()
            }
            PlainOp(Andi64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a & b).is_ok()
            }
            PlainOp(Ori64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a | b).is_ok()
            }
            PlainOp(Xori64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(a ^ b).is_ok()
            }
            PlainOp(Invi64) => {
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.push_int(!a).is_ok()
            }
            PlainOp(Lesi64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.push_bool(a < b).is_ok()
            }
            PlainOp(Grti64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.push_bool(a > b).is_ok()
            }
            PlainOp(Eqi64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.push_bool(a == b).is_ok()
            }
            PlainOp(Neqi64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.push_bool(a != b).is_ok()
            }
            PlainOp(Incf64) => {
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine.state.push_float(a + 1.0).is_ok()
            }
            PlainOp(Decf64) => {
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine.state.push_float(a - 1.0).is_ok()
            }
            PlainOp(Addf64) => {
                let b = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine.state.push_float(a + b).is_ok()
            }
            PlainOp(Subf64) => {
                let b = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine.state.push_float(a - b).is_ok()
            }
            PlainOp(Mulf64) => {
                let b = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine.state.push_float(a * b).is_ok()
            }
            PlainOp(Divf64) => {
                let b = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine.state.push_float(a / b).is_ok()
            }
            PlainOp(Remf64) => {
                let b = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine.state.push_float(a % b).is_ok()
            }
            PlainOp(Negf64) => {
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine.state.push_float(-a).is_ok()
            }
            PlainOp(Absf64) => {
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine.state.push_float(a.abs()).is_ok()
            }
            PlainOp(Powif64) => {
                let b = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine
                    .state
                    .push_float(a.powi(if b <= i32::max_value() as i64 &&
                                          b >= i32::min_value as i64 {
                                           b as i32
                                       } else {
                                           1
                                       }))
                    .is_ok()
            }
            PlainOp(Powff64) => {
                let b = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine.state.push_float(a.powf(b)).is_ok()
            }
            PlainOp(Lesf64) => {
                let b = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine.state.push_bool(a < b).is_ok()
            }
            PlainOp(Grtf64) => {
                let b = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine.state.push_bool(a > b).is_ok()
            }
            PlainOp(Eqf64) => {
                let b = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine.state.push_bool(a == b).is_ok()
            }
            PlainOp(Neqf64) => {
                let b = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine.state.push_bool(a != b).is_ok()
            }
            PlainOp(Andb) => {
                let b = machine.state.pop_bool().unwrap_or(false);
                let a = machine.state.pop_bool().unwrap_or(false);
                machine.state.push_bool(a && b).is_ok()
            }
            PlainOp(Orb) => {
                let b = machine.state.pop_bool().unwrap_or(false);
                let a = machine.state.pop_bool().unwrap_or(false);
                machine.state.push_bool(a || b).is_ok()
            }
            PlainOp(Eqb) => {
                let b = machine.state.pop_bool().unwrap_or(false);
                let a = machine.state.pop_bool().unwrap_or(false);
                machine.state.push_bool(a == b).is_ok()
            }
            PlainOp(Neqb) => {
                let b = machine.state.pop_bool().unwrap_or(false);
                let a = machine.state.pop_bool().unwrap_or(false);
                machine.state.push_bool(a != b).is_ok()
            }
            PlainOp(Notb) => {
                let a = machine.state.pop_bool().unwrap_or(false);
                machine.state.push_bool(!a).is_ok()
            }
            PlainOp(Itof) => {
                let a = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.push_float(a as f64).is_ok()
            }
            PlainOp(Ftoi) => {
                use std::num::FpCategory;
                let a = machine
                    .state
                    .pop_float()
                    .unwrap_or_else(&mut machine.float_handler);
                machine
                    .state
                    .push_int(match a.classify() {
                                  FpCategory::Normal => a as i64,
                                  FpCategory::Zero => 0,
                                  _ => (machine.int_handler)(),
                              })
                    .is_ok()
            }
            PlainOp(Rotins) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.rot_ins((pos & 0xFFFFFFFF) as usize)
            }
            PlainOp(Roti64) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.rot_int((pos & 0xFFFFFFFF) as usize)
            }
            PlainOp(Rotf64) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.rot_float((pos & 0xFFFFFFFF) as usize)
            }
            PlainOp(Rotb) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.rot_bool((pos & 0xFFFFFFFF) as usize)
            }
            PlainOp(Rotinsv) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.rot_ins_vec((pos & 0xFFFFFFFF) as usize)
            }
            PlainOp(Roti64v) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.rot_int_vec((pos & 0xFFFFFFFF) as usize)
            }
            PlainOp(Rotf64v) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.rot_float_vec((pos & 0xFFFFFFFF) as usize)
            }
            PlainOp(Copyins) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                if let Some(copy) = machine.state.copy_ins((pos & 0xFFFFFFFF) as usize) {
                    machine.state.push_ins(copy).is_ok()
                } else {
                    false
                }
            }
            PlainOp(Copyi64) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                if let Some(copy) = machine.state.copy_int((pos & 0xFFFFFFFF) as usize) {
                    machine.state.push_int(copy).is_ok()
                } else {
                    false
                }
            }
            PlainOp(Copyf64) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                if let Some(copy) = machine.state.copy_float((pos & 0xFFFFFFFF) as usize) {
                    machine.state.push_float(copy).is_ok()
                } else {
                    false
                }
            }
            PlainOp(Copyb) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                if let Some(copy) = machine.state.copy_bool((pos & 0xFFFFFFFF) as usize) {
                    machine.state.push_bool(copy).is_ok()
                } else {
                    false
                }
            }
            PlainOp(Copyinsv) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                if let Some(copy) = machine.state.copy_ins_vec((pos & 0xFFFFFFFF) as usize) {
                    machine.state.push_ins_vec(copy).is_ok()
                } else {
                    false
                }
            }
            PlainOp(Copyi64v) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                if let Some(copy) = machine.state.copy_int_vec((pos & 0xFFFFFFFF) as usize) {
                    machine.state.push_int_vec(copy).is_ok()
                } else {
                    false
                }
            }
            PlainOp(Copyf64v) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                if let Some(copy) = machine.state.copy_float_vec((pos & 0xFFFFFFFF) as usize) {
                    machine.state.push_float_vec(copy).is_ok()
                } else {
                    false
                }
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
                machine
                    .state
                    .push_exe(BasicBlock(if decider { b0 } else { b1 }))
                    .is_ok()
            }
        }
    }
}

