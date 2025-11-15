use crate::types::{DiskScheduler, DiskConfig, DiskRequest, SimulationResult};

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
}

impl DiskScheduler for SCAN {
    fn schedule(
        &self, 
        requests: &[DiskRequest], 
        config: &DiskConfig
    ) -> SimulationResult {
        let mut result = SimulationResult::new(self.name());

        if requests.is_empty() {
            return result;
        }

        let mut current_position = config.initial_position;
        let mut total_seek_time = 0;

        // Let's separate requests into left and right of initial position
        let mut left_requests: Vec<u32> = requests
            .iter()
            .filter(|req| req.track < current_position)
            .map(|req| req.track)
            .collect();

        let mut right_requests: Vec<u32> = requests
            .iter()
            .filter(|req| req.track >= current_position)
            .map(|req| req.track)
            .collect();

        // Sort left in descending order, right in ascending order
        left_requests.sort_unstable();
        left_requests.reverse(); // Descending order

        right_requests.sort_unstable(); // Ascending

        match self.initial_direction {
            ScanDirection::RIGHT => {
                for &track in &right_requests {
                    let seek_distance = current_position.abs_diff(track);
                    total_seek_time += seek_distance;
                    result.seek_sequence.push(track);
                    current_position = track;
                }

                // Go to left if necessary and then, process leftwards
                if current_position < config.max_track && !left_requests.is_empty() {
                    total_seek_time += current_position.abs_diff(config.max_track);
                    current_position = config.max_track;
                }

                for &track in &left_requests {
                    let seek_distance = current_position.abs_diff(track);
                    total_seek_time += seek_distance;
                    result.seek_sequence.push(track);
                    current_position = track;
                }
            }

            ScanDirection::LEFT => {
                for &track in &left_requests {
                    let seek_distance = current_position.abs_diff(track);
                    total_seek_time += seek_distance;
                    result.seek_sequence.push(track);
                    current_position = track;
                }

                // Go to beginning if necessary and then, process rightwards
                if current_position > 0 && !right_requests.is_empty() {
                    total_seek_time += current_position;
                    current_position = 0
                }

                for &track in &right_requests {
                    let seek_distance = current_position.abs_diff(track);
                    total_seek_time += seek_distance;
                    result.seek_sequence.push(track);
                    current_position = track;
                }
            }
        }

        result.total_seek_time = total_seek_time;
        result.requests_processed = requests.len();
        result.average_seek_time = total_seek_time as f64 / requests.len() as f64;

        result
    }

    fn name(&self) -> String {
        format!("SCAN Algorithm (Initial Direction: {:?})", self.initial_direction)
    }
}