use {Instruction, Machine};
use heapsize::HeapSizeOf;
use rand;
use vec::*;
use std::mem;

/// Instructions which have implicit parameters and are encodable with a single integer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
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
    /// ins: (a -- )
    Popins,
    /// int: (a -- )
    Popi64,
    /// float: (a -- )
    Popf64,
    /// bool: (a -- )
    Popb,
    /// ins vec: (a -- )
    Popinsv,
    /// int vec: (a -- )
    Popi64v,
    /// float vec: (a -- )
    Popf64v,

    // Vector Operations
    /// ins: (a -- )
    /// ins vec: (v -- a:v)
    Pushvins,
    /// int: (a -- )
    /// int vec: (v -- a:v)
    Pushvi64,
    /// float: (a -- )
    /// float vec: (v -- a:v)
    Pushvf64,
    /// ins vec: (_@(h:t) -- t)
    /// ins: ( -- h)
    Popvins,
    /// int vec: (_@(h:t) -- t)
    /// int: ( -- h)
    Popvi64,
    /// float vec: (_@(h:t) -- t)
    /// float: ( -- h)
    Popvf64,
    /// int: (i -- )
    /// ins vec: (v -- )
    /// ins: ( -- v[i])
    Readvins,
    /// int: (i -- v[i])
    /// int vec: (v -- )
    Readvi64,
    /// int: (i -- )
    /// float vec: (v -- )
    /// float: ( -- v[i])
    Readvf64,
    /// int: (i -- )
    /// ins: (e -- )
    /// ins vec: (v -- v)
    /// v[i] = e
    Writevins,
    /// int: (e i -- )
    /// int vec: (v -- v)
    /// v[i] = e
    Writevi64,
    /// int: (i -- )
    /// float: (e -- )
    /// float vec: (v -- v)
    /// v[i] = e
    Writevf64,

    // Auxiliary operations
    /// int: ( -- 0)
    Zeroi64,

    // Instruction construction
    /// int: (n -- )
    /// ins: ( -- PlainOp)
    CreatePlain,
    /// ins vec: (b -- )
    /// ins: ( -- BasicBlock(b))
    CreateBasicBlock,
    /// ins vec: (b -- )
    /// ins: ( -- Loop(b))
    CreateLoop,
    /// ins vec: (t f -- )
    /// ins: ( -- If(f, t))
    CreateIf,
    /// int: (n -- )
    /// ins: ( -- Pushi64(n))
    CreatePushi64,
    /// float: (n -- )
    /// ins: ( -- Pushf64(n))
    CreatePushf64,
    /// bool: (b -- )
    /// ins: ( -- Pushb(b))
    CreatePushb,
    /// int vec: (v -- )
    /// ins: ( -- Pushi64v(v))
    CreatePushi64v,
    /// float vec: (v -- )
    /// ins: ( -- Pushf64v(v))
    CreatePushf64v,

    // Execution control
    /// exe: (e -- )
    Return,
    /// exe: (e -- )
    /// ins: ( -- e)
    Yield,
    /// ins: (e -- )
    /// exe: ( -- e)
    Call,
    /// Does nothing
    Nop,

    // External communication
    /// ins: (i -- )
    Provide,
}

const TOTAL_PLAIN_INSTRUCTIONS: usize = 92;

impl rand::Rand for PlainOp {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        // NOTE: Change whenever PlainOp is changed.
        // TODO: Switch to proc macros 1.1 framework when compiler plugin is developed.
        unsafe { mem::transmute(rng.gen_range(0u8, TOTAL_PLAIN_INSTRUCTIONS as u8)) }
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
    Pushi64(i64),
    Pushf64(f64),
    Pushb(bool),
    Pushi64v(TrackedVec<i64>),
    Pushf64v(TrackedVec<f64>),
}

impl HeapSizeOf for SimpleInstruction {
    fn heap_size_of_children(&self) -> usize {
        use self::SimpleInstruction::*;
        match *self {
            PlainOp(_) => 0,
            BasicBlock(ref b) => b.heap_size_of_children(),
            Loop(ref l) => l.heap_size_of_children(),
            If(ref b0, ref b1) => b0.heap_size_of_children() + b1.heap_size_of_children(),
            Pushi64(_) => 0,
            Pushf64(_) => 0,
            Pushb(_) => 0,
            Pushi64v(ref v) => v.heap_size_of_children(),
            Pushf64v(ref v) => v.heap_size_of_children(),
        }
    }
}

impl<IH, IntH, FloatH> Instruction<IH, IntH, FloatH> for SimpleInstruction
    where IH: FnMut() -> Self,
          IntH: FnMut() -> i64,
          FloatH: FnMut() -> f64
{
    fn operate(self, machine: &mut Machine<Self, IH, IntH, FloatH>) -> (Option<Self>, bool) {
        use self::SimpleInstruction::*;
        use self::PlainOp::*;
        // The returned instruction, which most operations don't use.
        let mut ret_ins = None;
        let success = match self {
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
                    .push_int(a.pow((b.abs() & (0x7FFFFFFF)) as u32))
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
                    .push_int(a.rotate_left((b & (0x7FFFFFFF)) as u32))
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
                    .push_int(a.rotate_right((b & (0x7FFFFFFF)) as u32))
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
                    .push_int(a.checked_shl((b & (0x7FFFFFFF)) as u32)
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
                    .push_int(a.checked_shr((b & (0x7FFFFFFF)) as u32)
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
                machine.state.rot_ins((pos & 0x7FFFFFFF) as usize)
            }
            PlainOp(Roti64) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.rot_int((pos & 0x7FFFFFFF) as usize)
            }
            PlainOp(Rotf64) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.rot_float((pos & 0x7FFFFFFF) as usize)
            }
            PlainOp(Rotb) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.rot_bool((pos & 0x7FFFFFFF) as usize)
            }
            PlainOp(Rotinsv) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.rot_ins_vec((pos & 0x7FFFFFFF) as usize)
            }
            PlainOp(Roti64v) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.rot_int_vec((pos & 0x7FFFFFFF) as usize)
            }
            PlainOp(Rotf64v) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine.state.rot_float_vec((pos & 0x7FFFFFFF) as usize)
            }
            PlainOp(Copyins) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .copy_ins((pos & 0x7FFFFFFF) as usize)
                    .and_then(|copy| machine.state.push_ins(copy).ok())
                    .is_some()
            }
            PlainOp(Copyi64) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .copy_int((pos & 0x7FFFFFFF) as usize)
                    .and_then(|copy| machine.state.push_int(copy).ok())
                    .is_some()
            }
            PlainOp(Copyf64) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .copy_float((pos & 0x7FFFFFFF) as usize)
                    .and_then(|copy| machine.state.push_float(copy).ok())
                    .is_some()
            }
            PlainOp(Copyb) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .copy_bool((pos & 0x7FFFFFFF) as usize)
                    .and_then(|copy| machine.state.push_bool(copy).ok())
                    .is_some()
            }
            PlainOp(Copyinsv) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .copy_ins_vec((pos & 0x7FFFFFFF) as usize)
                    .and_then(|copy| machine.state.push_ins_vec(copy).ok())
                    .is_some()
            }
            PlainOp(Copyi64v) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .copy_int_vec((pos & 0x7FFFFFFF) as usize)
                    .and_then(|copy| machine.state.push_int_vec(copy).ok())
                    .is_some()
            }
            PlainOp(Copyf64v) => {
                let pos = machine
                    .state
                    .pop_int()
                    .unwrap_or_else(&mut machine.int_handler);
                machine
                    .state
                    .copy_float_vec((pos & 0x7FFFFFFF) as usize)
                    .and_then(|copy| machine.state.push_float_vec(copy).ok())
                    .is_some()
            }
            PlainOp(Popins) => machine.state.pop_ins().is_some(),
            PlainOp(Popi64) => machine.state.pop_int().is_some(),
            PlainOp(Popf64) => machine.state.pop_float().is_some(),
            PlainOp(Popb) => machine.state.pop_bool().is_some(),
            PlainOp(Popinsv) => machine.state.pop_ins_vec().is_some(),
            PlainOp(Popi64v) => machine.state.pop_int_vec().is_some(),
            PlainOp(Popf64v) => machine.state.pop_float_vec().is_some(),
            PlainOp(Pushvins) => {
                machine
                    .state
                    .pop_ins()
                    .and_then(|e| machine.state.push_ins_to_vec(e).ok())
                    .unwrap_or(false)
            }
            PlainOp(Pushvi64) => {
                machine
                    .state
                    .pop_int()
                    .and_then(|e| machine.state.push_int_to_vec(e).ok())
                    .unwrap_or(false)
            }
            PlainOp(Pushvf64) => {
                machine
                    .state
                    .pop_float()
                    .and_then(|e| machine.state.push_float_to_vec(e).ok())
                    .unwrap_or(false)
            }
            PlainOp(Popvins) => machine.state.pop_ins_from_vec().is_some(),
            PlainOp(Popvi64) => machine.state.pop_int_from_vec().is_some(),
            PlainOp(Popvf64) => machine.state.pop_float_from_vec().is_some(),
            PlainOp(Readvins) => {
                machine
                    .state
                    .pop_int()
                    .and_then(|ix| {
                                  machine
                                      .state
                                      .get_ins_from_vec((ix & 0x7FFFFFFF) as usize)
                              })
                    .and_then(|e| machine.state.push_ins(e).ok())
                    .is_some()
            }
            PlainOp(Readvi64) => {
                machine
                    .state
                    .pop_int()
                    .and_then(|ix| {
                                  machine
                                      .state
                                      .get_int_from_vec((ix & 0x7FFFFFFF) as usize)
                              })
                    .and_then(|e| machine.state.push_int(e).ok())
                    .is_some()
            }
            PlainOp(Readvf64) => {
                machine
                    .state
                    .pop_int()
                    .and_then(|ix| {
                                  machine
                                      .state
                                      .get_float_from_vec((ix & 0x7FFFFFFF) as usize)
                              })
                    .and_then(|e| machine.state.push_float(e).ok())
                    .is_some()
            }
            PlainOp(Writevins) => {
                machine
                    .state
                    .pop_int()
                    .and_then(|ix| machine.state.pop_ins().map(|e| (ix, e)))
                    .and_then(|(ix, e)| {
                                  machine
                                      .state
                                      .write_ins_to_vec((ix & 0x7FFFFFFF) as usize, e)
                                      .ok()
                              })
                    .unwrap_or(false)
            }
            PlainOp(Writevi64) => {
                machine
                    .state
                    .pop_int()
                    .and_then(|ix| machine.state.pop_int().map(|e| (ix, e)))
                    .map_or(false, |(ix, e)| {
                        machine
                            .state
                            .write_int_to_vec((ix & 0x7FFFFFFF) as usize, e);
                        true
                    })
            }
            PlainOp(Writevf64) => {
                machine
                    .state
                    .pop_int()
                    .and_then(|ix| machine.state.pop_float().map(|e| (ix, e)))
                    .map_or(false, |(ix, e)| {
                        machine
                            .state
                            .write_float_to_vec((ix & 0x7FFFFFFF) as usize, e);
                        true
                    })
            }
            PlainOp(Zeroi64) => machine.state.push_int(0).is_ok(),
            PlainOp(CreatePlain) => {
                machine
                    .state
                    .pop_int()
                    .map(|n| (n & 0x7FFFFFFF) as usize)
                    .and_then(|n| if n < TOTAL_PLAIN_INSTRUCTIONS {
                                  Some(unsafe { mem::transmute(n as u8) })
                              } else {
                                  None
                              })
                    .map(PlainOp)
                    .and_then(|ins| machine.state.push_ins(ins).ok())
                    .is_some()
            }
            PlainOp(CreateBasicBlock) => {
                machine
                    .state
                    .pop_ins_vec()
                    .map(|v| BasicBlock(v.into_iter()))
                    .and_then(|ins| machine.state.push_ins(ins).ok())
                    .is_some()
            }
            PlainOp(CreateLoop) => {
                machine
                    .state
                    .pop_ins_vec()
                    .map(|v| Loop(v.into_cycle_iter()))
                    .and_then(|ins| machine.state.push_ins(ins).ok())
                    .is_some()
            }
            PlainOp(CreateIf) => {
                machine
                    .state
                    .pop_ins_vec()
                    .and_then(|vf| machine.state.pop_ins_vec().map(|vt| (vf, vt)))
                    .map(|(vf, vt)| If(vf.into_iter(), vt.into_iter()))
                    .and_then(|ins| machine.state.push_ins(ins).ok())
                    .is_some()
            }
            PlainOp(CreatePushi64) => {
                machine
                    .state
                    .pop_int()
                    .map(Pushi64)
                    .and_then(|ins| machine.state.push_ins(ins).ok())
                    .is_some()
            }
            PlainOp(CreatePushf64) => {
                machine
                    .state
                    .pop_float()
                    .map(Pushf64)
                    .and_then(|ins| machine.state.push_ins(ins).ok())
                    .is_some()
            }
            PlainOp(CreatePushb) => {
                machine
                    .state
                    .pop_bool()
                    .map(Pushb)
                    .and_then(|ins| machine.state.push_ins(ins).ok())
                    .is_some()
            }
            PlainOp(CreatePushi64v) => {
                machine
                    .state
                    .pop_int_vec()
                    .map(Pushi64v)
                    .and_then(|ins| machine.state.push_ins(ins).ok())
                    .is_some()
            }
            PlainOp(CreatePushf64v) => {
                machine
                    .state
                    .pop_float_vec()
                    .map(Pushf64v)
                    .and_then(|ins| machine.state.push_ins(ins).ok())
                    .is_some()
            }
            PlainOp(Return) => machine.state.pop_exe().is_some(),
            PlainOp(Yield) => {
                machine
                    .state
                    .pop_exe()
                    .and_then(|e| machine.state.push_ins(e).ok())
                    .is_some()
            }
            PlainOp(Call) => {
                machine
                    .state
                    .pop_ins()
                    .and_then(|e| machine.state.push_exe(e).ok())
                    .is_some()
            }
            PlainOp(Nop) => true,
            PlainOp(Provide) => {
                ret_ins = machine.state.pop_ins();
                ret_ins.is_some()
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
            Pushi64(n) => machine.state.push_int(n).is_ok(),
            Pushf64(n) => machine.state.push_float(n).is_ok(),
            Pushb(b) => machine.state.push_bool(b).is_ok(),
            Pushi64v(v) => machine.state.push_int_vec(v).is_ok(),
            Pushf64v(v) => machine.state.push_float_vec(v).is_ok(),
        };
        (ret_ins, success)
    }
}

