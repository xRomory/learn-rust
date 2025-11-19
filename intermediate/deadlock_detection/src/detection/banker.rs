use crate::models::SystemState;

#[derive(Debug, Clone)]
pub struct BankerAlgorithm {
    state: SystemState,
}

impl BankerAlgorithm {
    pub fn new(state: SystemState) -> Self {
        Self { state }
    }

    pub fn request_resource(
        &mut self,
        process_id: usize,
        request: Vec<usize>
    ) -> Result<(), String> {
        if process_id >= self.state.processes {
            return Err(format!("Invalid process ID: {}", process_id));
        }

        if request.len() != self.state.resources {
            return Err(format!("Request must have {} resources", self.state.resources));
        }

        // Check if request exceeds need
        for j in 0..self.state.resources {
            if request[j] > self.state.need[process_id][j] {
                return Err(format!("Process {} must wait - resources not available", process_id));
            }
        }

        // Try to allocate resources temporarily
        let mut temp_state = self.state.clone();

        for j in 0..self.state.resources {
            temp_state.available[j] -= request[j];  // Remove from available
            temp_state.allocation[process_id][j] += request[j]; // Add to allocation
            temp_state.need[process_id][j] -= request[j];   // Reduce need
        }

        if temp_state.is_safe_state() {
            self.state = temp_state;
            Ok(())
        } else {
            Err("Request would lead to unsafe state".to_string())
        }
    }
}