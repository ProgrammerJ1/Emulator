use std::{boxed::Box,ops::RangeInclusive};
use bitvec::{order::Msb0,vec::BitVec};
pub enum InstructionSize {
    Fixed(u64),
    Variable(RangeInclusive<u64>)
}
pub struct InstructionMode {
    size: InstructionSize,
}