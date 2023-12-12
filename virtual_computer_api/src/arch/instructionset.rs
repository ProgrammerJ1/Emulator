use std::{boxed::Box,ops::RangeInclusive};
use bitvec::{slice::BitSlice};
pub enum InstructionSize {
    Fixed(u64),
    Variable(RangeInclusive<u64>)
}
pub struct InstructionFormat {
    param_sizes: Box<[usize]>,
    short_circuiting: bool,
    pub validate: fn (&self,bits:&BitSlice)->(bool,usize)
}
impl InstructionFormat {
    pub fn new(param_sizes:&[usize],short_circuiting:bool,validator: fn (&self,bits:&BitSlice)->(bool,usize))->Self {
        Self{param_sizes,short_circuiting,validator}
    }
}
pub struct InstructionMode {
    size: InstructionSize,
    formats: BTreeMap<u128,InstructionFormat>
}
impl InstructionMode {
    pub fn initalize_context(size:InstructionSize,formats:Vec<InstructionFormats>)->Self {
        Self { size, formats}
    }
    pub fn pattern_match(&self,bits:&BitSlice)->&I {
        ;
    }
}
