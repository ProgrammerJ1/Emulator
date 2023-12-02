pub enum Endianness {
    Little,
    Big
}
pub struct HostContext {
    pub endianess: Endianness
}
impl HostContext {
    fn new(endianess:Endianness)->Self {
        return Self {endianess}
    }
}
fn get_host_context()->HostContext {
    let endianness: Endianness;
    if cfg!(target_endian = "big") {
        endianness=Endianness::Big;
    } else {
        endianness=Endianness::Little;
    }
    HostContext::new(endianness)
}
