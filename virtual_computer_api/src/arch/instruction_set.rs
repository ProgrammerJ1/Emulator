use std::{boxed::Box,ops::RangeInclusive};
use bitvec::{slice::BitSlice};
pub enum InstructionError {
    EmptySet,
    NotWithinISASize,
    NoMatches,
}
#[derive(PartialEq, Eq)]
pub enum InstructionSize {
    Fixed(usize),
    Variable(RangeInclusive<usize>)
}
impl InstructionSize {
    pub fn within_size(&self,bits:&BitSlice)->bool {
        match self {
            Self::Fixed(size)=>return size.clone()==bits.len(),
            Self::Variable(size_range)=>return size_range.clone().contains(&bits.len())
        }
    }
}
#[derive(PartialEq, Eq)]
pub struct InstructionFormat {
    param_sizes: Box<[usize]>,
    short_circuiting: bool,
    pub validate: fn (&Self,bits:&BitSlice)->bool
}
impl InstructionFormat {
    pub fn new(param_sizes:Vec<usize>,short_circuiting:bool,validator: fn (&Self,bits:&BitSlice)->bool)->Self {
        Self{param_sizes:param_sizes.into_boxed_slice(),short_circuiting,validate: validator}
    }
}
pub struct InstructionMode {
    size: InstructionSize,
    formats: Vec<InstructionFormat>
}
impl InstructionMode {
    pub fn initalize_context(size:InstructionSize,formats:Vec<InstructionFormat>) {
        //
    }
    #[inline(always)]
    pub fn initalize_context_unchecked(size:InstructionSize,formats:Vec<InstructionFormat>)->Self {
        Self { size, formats}
    }
    pub fn pattern_match<'a>(&'a self,bits:&BitSlice)->Result<&'a InstructionFormat,InstructionError> {
        if self.formats.is_empty() {
            return Err(InstructionError::EmptySet);
        } else if !self.size.within_size(bits) {
            return Err(InstructionError::NotWithinISASize);
        } else {
            let mut best_match: Option<&'a InstructionFormat>=None;
            for format in &self.formats {
                if (format.validate)(format,bits) {
                    best_match=Some(format);
                    if format.short_circuiting {
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
