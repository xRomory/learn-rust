pub mod avoidance;
pub mod preemption;

pub use avoidance::DeadlockAvoidance;
pub use preemption::PreemptiveAllocator;

pub trait DeadlockPreventor {
    fn can_allocate(&self, process_id: usize, request: &[usize]) -> bool;
    fn allocate(&mut self, process_id: usize, request: Vec<usize>) -> Result<(), String>;
}