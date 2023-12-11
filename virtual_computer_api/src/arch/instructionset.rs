use std::{boxed::Box,ops::RangeInclusive};
use bitvec::{slice::BitSlice};
pub enum InstructionSize {
    Fixed(u64),
    Variable(RangeInclusive<u64>)
}
pub struct InstructionMode {
    size: InstructionSize,
    instruction_formats: Box<[fn (&Self,bits:BitSlice)->(bool,bool)]>
}
impl InstructionMode {
    pub fn initalize_context(size:InstructionSize,formats:Vec<fn (&Self,bits:BitSlice)->(bool,bool)>)->Self {
        Self { size, instruction_formats: formats.into_boxed_slice()}
    }

}