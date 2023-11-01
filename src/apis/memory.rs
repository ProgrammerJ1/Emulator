use std::boxed::Box;
enum Memory {
    RAM(Box<mut [u8]>)
    ROM(Box<[u8]>)
}
pub struct MemoryRegion<WordSize,AddressSize> {
    address: [u8; AddressSize];
    memory: Memory;
}
