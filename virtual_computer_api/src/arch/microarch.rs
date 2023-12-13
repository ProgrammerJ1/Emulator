use instructionset::InstructionMode
pub trait Microarch<'a> {
    //With any of implementor, just define a collection and return its reference, inline the function
    pub fn get_isa(&self)->&'a Vec<InstructionMode>;
    pub fn get_current_isa_mode(&self)->&'a InstructionMode;
    pub fn set_isa_mode(&self,index: usize)->bool;
}
