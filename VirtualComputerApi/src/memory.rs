use std::vec::Vec;
pub enum Memory<'guestos> {
    RAM(&'guestos mut [u8]),
    ROM(&'guestos [u8]),
    IOMMU(&'guestos [u8]),
    WIOMMU(&'guestos mut [u8])
}
pub struct MemoryRegion<'guestos> {
    pub address: usize,
    pub memory: Memory<'guestos>,
    pub downward: bool
}
pub struct MemorySubsystem<'guestos> {
    pub addressable_units: Vec<usize>,
    pub mmu_enabled: bool,
    pub phys_memory_regions: Vec<MemoryRegion<'guestos>>
}
