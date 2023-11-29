pub enum Endianness {
    Little,
    Big
}
pub struct SystemContext {
    pub endianess: Endianness
}
impl SystemContext {
    fn new(endianess:Endianness)->Self {
        return Self {endianess}
    }
}
fn get_host_context()->SystemContext {
    let endianness: Endianness;
    if cfg!(target_endian = "big") {
        endianness=Endianness::Big;
    } else {
        endianness=Endianness::Little;
    }
    SystemContext::new(endianness)
}
