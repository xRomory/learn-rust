use crate::{models::SystemState, prevention::DeadlockPreventor};

#[derive(Debug, Clone)]
pub struct DeadlockAvoidance {
    state: SystemState,
}

impl DeadlockAvoidance {
    pub fn new(state: SystemState) -> Self {
        Self { state }
    }

    pub fn get_state(&self) -> &SystemState {
        &self.state
    }

    fn would_be_safe(&self, process_id: usize, request: &[usize]) -> bool {
        let mut temp_state = self.state.clone();

        for j in 0..self.state.resources {
            if request[j] > temp_state.available[j] {
                return false;
            }

            temp_state.available[j] -= request[j];
            temp_state.allocation[process_id][j] += request[j];
            temp_state.need[process_id][j] -= request[j];
        }

        temp_state.is_safe_state()
    }
}

impl DeadlockPreventor for DeadlockAvoidance {
    fn can_allocate(
        &self,
        process_id: usize,
        request: &[usize]
    ) -> bool {
        if process_id >= self.state.processes || request.len() != self.state.resources {
            return false;
        }

        // Check if request exceeds need
        for j in 0..self.state.resources {
            if request[j] > self.state.need[process_id][j] {
                return false;
            }
        }

        // Check if request exceeds available
        for j in 0..self.state.resources {
            if request[j] > self.state.available[j] {
                return false;
            }
        }

        // Check if allocation would lead to safe state
        self.would_be_safe(process_id, request)
    }

    fn allocate(
        &mut self,
        process_id: usize,
        request: Vec<usize>,
    ) -> Result<(), String> {
        if !self.can_allocate(process_id, &request) {
            return Err("Allocation would lead to unsafe state".to_string());
        }

        for j in 0..self.state.resources {
            self.state.available[j] -= request[j];
            self.state.allocation[process_id][j] += request[j];
            self.state.need[process_id][j] -= request[j];
        }

        Ok(())
    }
}