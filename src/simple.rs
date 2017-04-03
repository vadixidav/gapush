use {Instruction, Machine};
use num::FromPrimitive;
use rand;

enum_from_primitive! {
/// Instructions which have implicit parameters and are encodable with a single integer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlainOp {
    Addi64,
}
}

impl rand::Rand for PlainOp {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        Self::from_u32(rng.gen_range(0, 1)).unwrap()
    }
}

pub enum SimpleInstruction {
    PlainOp(PlainOp),
}

