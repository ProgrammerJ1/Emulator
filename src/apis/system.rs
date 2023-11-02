use crate::apis::memory::MemorySubsystem;
pub struct SystemResources {
    pub cpu: Procressor,
    pub memory: MemorySubsystem,
    pub io: IOSubsystem
}
