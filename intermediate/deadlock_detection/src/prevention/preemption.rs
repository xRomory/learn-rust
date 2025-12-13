use crate::{models::SystemState, prevention::DeadlockPreventor};

#[derive(Debug, Clone)]
pub struct PreemptiveAllocator {
    pub state: SystemState
}

impl PreemptiveAllocator {
    pub fn new(state: SystemState) -> Self {
        Self { state }
    }

    fn preempt_resources(&mut self, victim_process: usize) -> Vec<usize> {
        let resources = self.state.allocation[victim_process].clone();

        for j in 0..self.state.resources {
            self.state.available[j] += self.state.allocation[victim_process][j];
            self.state.allocation[victim_process][j] = 0;
            self.state.need[victim_process][j] = self.state.max[victim_process][j];
        }

        resources
    }

    fn find_victim_process(
        &self,
        requesting_process: usize,
        request: &[usize]
    ) -> Option<usize> {
        (0..self.state.processes)
            .filter(|&i| i != requesting_process)
            .find(|&i| {
                (0..self.state.resources)
                    .all(|j| self.state.allocation[i][j] >= request[j])
            })
    }
}

impl DeadlockPreventor for PreemptiveAllocator {
    fn can_allocate(
        &self,
        _process_id: usize,
        _request: &[usize]
    ) -> bool {
        // With preemption, we can always allocate by preempting resources
        true
    }

    fn allocate(
        &mut self,
        process_id: usize,
        request: Vec<usize>
    ) -> Result<(), String> {
        let mut temp_state = self.state.clone();
        let mut can_allocate_nomal = true;

        for j in 0..self.state.resources {
            if request[j] > temp_state.available[j] {
                can_allocate_nomal = false;
                break;
            }

            temp_state.available[j] -= request[j];
            temp_state.allocation[process_id][j] += request[j];
            temp_state.need[process_id][j] -= request[j];
        }

        if can_allocate_nomal && temp_state.is_safe_state() {
            self.state = temp_state;
            return Ok(());
        }

        // If normal allocation fails, try preemption
        if let Some(victim) = self.find_victim_process(process_id, &request) {
            println!("Preempting resources from process {}", victim);
            self.preempt_resources(victim);

            // Try allocation again
            for j in 0..self.state.processes {
                if request[j] > self.state.available[j] {
                    return Err("Cannot allocate even with preemption".to_string());
                }
                self.state.available[j] -= request[j];
                self.state.allocation[process_id][j] += request[j];
                self.state.need[process_id][j] -= request[j];
            }

            Ok(())
        } else {
            Err("No suitable victim process found for preemption".to_string())
        }
    }
}