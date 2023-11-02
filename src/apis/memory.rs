use std::boxed::Box;
enum Memory {
    RAM(&mut [u8])
    ROM(&[u8])
    IOMMU(&[u8])
    WIOMMU(&mut [u8])
}
pub struct MemoryRegion<WordSize,AddressSize> {
    address: [u8; AddressSize];
    memory: Memory;
}
