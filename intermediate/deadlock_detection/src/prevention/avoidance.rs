use crate::models::SystemState;

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