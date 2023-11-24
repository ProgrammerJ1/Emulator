use byteorder::NativeEndian;
pub enum Endianess {
    Little,
    Mixed,
    Big
}
pub struct SystemContext {
    endianess: Endianess
}
fn get_host_context()->SystemContext {
    SystemContext { endianess: NativeEndian }
}