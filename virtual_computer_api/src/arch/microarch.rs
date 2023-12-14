use super::instruction_set::InstructionMode;
pub trait Microarch<'a> {
    //With any of implementor, just define a collection and return its reference, inline the function
    fn get_isa(&self)->&'a Vec<InstructionMode>;
    fn get_current_isa_mode(&self)->&'a InstructionMode;
    fn set_isa_mode(&self,index: usize)->bool;
}
