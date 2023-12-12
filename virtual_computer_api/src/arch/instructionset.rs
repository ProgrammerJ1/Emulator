use std::{boxed::Box,ops::RangeInclusive};
use bitvec::{slice::BitSlice};
pub enum InstructionError {
    EmptySet,
    NotWithinISASize,
    NoMatches,
}
pub enum InstructionSize {
    Fixed(usize),
    Variable(RangeInclusive<usize>)
}
impl InstructionSize {
    pub fn within_size(&self,bits:&BitSlice)->bool {
        match self {
            Fixed(size)=>return size==bits.len(),
            Variable(size_range)=>return size_range.contains(bits.len())
        }
    }
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
    formats: Vec<InstructionFormat>
}
impl InstructionMode {
    pub fn initalize_context(size:InstructionSize,formats:Vec<InstructionFormats>)->Self {
        Self { size, formats}
    }
    pub fn pattern_match<'a>(&self,bits:&BitSlice)->Result<&'a InstructionFormat,InstructionError> {
        if self.formats.is_empty() {
            return Err(InstructionError::EmptySet);
        } else if (!self.size.within_size(bits)) {
            return Err(InstructionError::NotWithinISASize);
        } else {
            let mut best_match: Option<&'a InstructionFormat>=None;
            for format in self.formats {
                if (format.validate(bits)) {
                    best_match=Some(format);
                    if self.short_circuiting {
                        break;
                    }
                }
            }
            match best_match {
                Some(format)=>return Ok(format),
                None=>return Err(InstructionError::NoMatches)
            }
        }
    }
}
