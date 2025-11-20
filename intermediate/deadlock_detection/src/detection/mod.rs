pub mod banker;

pub use banker::BankerAlgorithm;

pub trait DeadlockDetector {
    fn detect_deadlock(&self) -> Option<Vec<usize>>;
    fn is_safe_state(&self) -> bool;
}