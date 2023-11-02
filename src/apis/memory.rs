pub enum Memory {
    RAM(&mut [u8]),
    ROM(&[u8]),
    IOMMU(&[u8]),
    WIOMMU(&mut [u8])
}
pub struct MemoryRegion<WordSize,AddressSize> {
    pub address: [u8; (AddressSize/8)+(AddressSize%8)];
    pub memory: Memory;
    pub downward: bool;
}
pub struct MemorySubsystem {
    d
