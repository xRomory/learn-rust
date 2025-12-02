pub mod avoidance;

pub trait DeadlockPreventor {
    fn can_allocate(&self, process_id: usize, request: &[usize]) -> bool;
    fn allocate(&mut self, process_id: usize, request: Vec<usize>) -> Result<(), String>;
}