use std::{boxed::Box,ops::RangeInclusive};
use bitvec::{slice::BitSlice};
pub enum InstructionSize {
    Fixed(u64),
    Variable(RangeInclusive<u64>)
}
pub struct InstructionFormat {
    param_sizes: Box<[usize]>,
    pub validate: fn (&self,bits:&BitSlice)->(bool,bool,usize)
}
impl InstructionFormat {
    pub fn new(param_sizes:&[usize],validator: fn (&self,bits:&BitSlice)->(bool,bool,usize))->Self {
        Self{param_sizes,validator}
    }
}
pub struct InstructionMode {
    size: InstructionSize,
    instruction_formats: BTreeMap<u128,InstructionFormat>
}
impl InstructionMode {
    pub fn initalize_context(size:InstructionSize,formats:Vec<fn (&Self,bits:&BitSlice)->(bool,bool,usize)>)->Self {
        Self { size, instruction_formats: }
    }
    pub fn pattern_match(&self,bits:&BitSlice)->&I {
        ;
    }
}
