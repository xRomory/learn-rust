use crate::{types::DiskScheduler, DiskConfig, DiskRequest, SimulationResult};

#[derive(Debug, Clone, Copy)]
pub enum ScanDirection {
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct SCAN {
    pub initial_direction: ScanDirection
}

impl SCAN {
    pub fn new(initial_direction: ScanDirection) -> Self {
        Self { initial_direction }
    }

    pub fn default() -> Self {
        Self::new(ScanDirection::RIGHT)
    }
}

impl DiskScheduler for SCAN {
    fn schedule(
        &self, 
        requests: &[DiskRequest], 
        config: &DiskConfig
    ) -> SimulationResult {
        let mut result = SimulationResult::new(self.name());

        result
    }

    fn name(&self) -> String {
        format!("SCAN Algorithm (Initial Direction: {:?})", self.initial_direction)
    }
}