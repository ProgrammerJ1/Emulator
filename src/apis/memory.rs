use std::vec::Vec
pub enum Memory {
    RAM(&mut [u8]),
    ROM(&[u8]),
    IOMMU(&[u8]),
    WIOMMU(&mut [u8])
}
pub struct MemoryRegion<WordSize> {
    pub address: usize,
    pub memory: Memory,
    pub downward: bool
}
pub struct MemorySubsystem<WordSize,AddressSize> {
    pub addressable: Vec<usize>,
    pub mmu_enabled: bool,
    pub phys_memory_regions: Vec<MemoryRegion<WordSize,AddressSize>>
