use rand::Rng;

use crate::{
    models::SystemState,
    detection::{BankerAlgorithm},
    prevention::{DeadlockAvoidance, PreemptiveAllocator, DeadlockPreventor},
};

#[derive(Debug)]
pub struct DeadlockSimulator {
    state: SystemState,
    max_requests: usize,
}

impl DeadlockSimulator {
    pub fn new(
        available: Vec<usize>,
        max: Vec<Vec<usize>>,
        allocation: Vec<Vec<usize>>
    ) -> Self {
        let state = SystemState::new(available, max, allocation);

        Self {
            state,
            max_requests: 10
        }
    }
}