use std::vec::Vec;
use crate::system::SystemResources;
use crate::host_context::HostContext;
pub enum Memory<'guestos> {
    RAM(&'guestos mut [u8]),
    ROM(&'guestos [u8]),
    MMIO{memory: &'guestos mut [u8],read: fn (system_resources:SystemResources,host_context:HostContext),write:fn (system_resources:SystemResources,host_context:HostContext)}
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
