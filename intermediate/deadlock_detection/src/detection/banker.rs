use crate::{detection::DeadlockDetector, models::SystemState};

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

    pub fn get_safe_sequence(&self) -> Option<Vec<usize>> {
        let mut work = self.state.available.clone();
        let mut finish = vec![false; self.state.processes];
        let mut safe_sequence = Vec::new();

        for _ in 0..self.state.processes {
            let mut found = false;

            for i in 0..self.state.processes {
                if !finish[i] && self.can_allocate(i, &work) {
                    for j in 0..self.state.processes {
                        work[j] += self.state.allocation[i][j];
                    }

                    finish[i] = true;
                    safe_sequence.push(i);
                    found = true;
                }
            }

            if !found { break; }
        }

        if safe_sequence.len() == self.state.processes {
            Some(safe_sequence)
        } else {
            None
        }
    }

    fn can_allocate(
        &self,
        process: usize,
        work: &[usize],
    ) -> bool {
        (0..self.state.resources).all(|j| self.state.need[process][j] <= work[j])
    }
}

impl DeadlockDetector for BankerAlgorithm {
    fn detect_deadlock(&self) -> Option<Vec<usize>> {
        match self.get_safe_sequence() {
            Some(_) => None,
            None => {
                let mut deadlocked = Vec::new();
                let safe_seq = self.get_safe_sequence().unwrap_or_default();

                for i in 0..self.state.processes {
                    if !safe_seq.contains(&i) {
                        deadlocked.push(i);
                    }
                }

                Some(deadlocked)
            }
        }
    }

    fn is_safe_state(&self) -> bool {
        self.get_safe_sequence().is_some()
    }
}