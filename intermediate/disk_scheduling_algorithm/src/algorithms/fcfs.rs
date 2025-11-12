use crate::types::{DiskScheduler, DiskConfig, DiskRequest, SimulationResult};

#[derive(Debug)]
pub struct BasicFCFS;

impl DiskScheduler for BasicFCFS {
    fn schedule(
        &self,
        requests: &[DiskRequest],
        config: &DiskConfig
    ) -> SimulationResult {
        let mut result = SimulationResult::new(self.name());
        let mut current_position = config.initial_position;

        for request in requests {
            let seek_distance = current_position.abs_diff(request.track);

            result.total_seek_time += seek_distance;
            result.seek_sequence.push(request.track);
            current_position = request.track;
        }

        result.requests_processed = requests.len();
        result.average_seek_time = if !requests.is_empty() {
            result.total_seek_time as f64 / requests.len() as f64
        } else {
            0.0
        };

        result
    }

    fn name(&self) -> String {
        "Basic FCFS Algorithm".to_string()
    }
}