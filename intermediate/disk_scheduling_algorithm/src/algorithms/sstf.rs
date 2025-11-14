use crate::types::{DiskConfig, DiskRequest, DiskScheduler, SimulationResult};

#[derive(Debug)]
pub struct SSTF;

impl DiskScheduler for SSTF {
    fn schedule(&self, requests: &[DiskRequest], config: &DiskConfig) -> SimulationResult {
        let mut result = SimulationResult::new(self.name());

        if requests.is_empty() {
            return result;
        }

        let mut remaining_requests: Vec<DiskRequest> = requests.to_vec();
        let mut current_position = config.initial_position;
        let mut total_seek_time = 0;

        while !remaining_requests.is_empty() {
            let (closest_index, closest_request, seek_distance) = remaining_requests
                .iter()
                .enumerate()
                .map(|(idx, req)| {
                    let distance = current_position.abs_diff(req.track);
                    (idx, req, distance)
                })
                .min_by_key(|&(_, _, distance)| distance)
                .unwrap();
            
            total_seek_time += seek_distance;
            result.seek_sequence.push(closest_request.track);
            current_position = closest_request.track;

            // Remove the processed request
            remaining_requests.remove(closest_index);
        }

        result.total_seek_time = total_seek_time;
        result.requests_processed = requests.len();
        result.average_seek_time = total_seek_time as f64 / requests.len() as f64;

        result
    }

    fn name(&self) -> String {
        "SSTF (Shortest Seek Time First)".to_string()
    }
}