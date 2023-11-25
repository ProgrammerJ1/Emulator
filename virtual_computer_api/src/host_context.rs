pub enum Endianess {
    Little,
    Big
}
pub struct SystemContext {
    endianess: Endianess
}
impl SystemContext {
    fn new(endianess:Endianess)->Self {
        return Self {endianess}
    }
}
fn get_host_context()->SystemContext {
    let endianess: Endianess;
    if cfg!(target_endian = "big") {
        endianess=Endianess::Big;
    } else {
        endianess=Endianess::Little;
    }
    SystemContext::new(endianess)
}