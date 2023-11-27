pub enum Endianness {
    Little,
    Big
}
pub struct SystemContext {
    pub endianess: Endianness
}
impl SystemContext {
    fn new(endianness:Endianness)->Self {
        return Self {endianess}
    }
}
fn get_host_context()->SystemContext {
    let endianness: Endianess;
    if cfg!(target_endian = "big") {
        endianess=Endianness::Big;
    } else {
        endianess=Endianness::Little;
    }
    SystemContext::new(endianness)
}
