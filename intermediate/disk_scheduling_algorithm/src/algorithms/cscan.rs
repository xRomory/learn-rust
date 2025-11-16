use crate::types::{DiskScheduler, DiskRequest, DiskConfig, SimulationResult};

#[derive(Debug, Clone, Copy)]
pub enum CScanDirection {
    LEFT,
    RIGHT
}

#[derive(Debug)]
pub struct CSCAN {
    pub direction: CScanDirection
}

impl CSCAN {
    pub fn new(direction: CScanDirection) -> Self {
        Self { direction }
    }
}

impl DiskScheduler for CSCAN {
    fn schedule(
        &self,
        requests: &[DiskRequest],
        config: &DiskConfig
    ) -> SimulationResult {
        let mut result = SimulationResult::new(self.name());

        if requests.is_empty() {
            return result;
        }

        let initial_position = config.initial_position;
        let mut current_position = initial_position;
        let mut total_seek_time = 0;

        // Extract and sort all unique tracks
        let mut tracks: Vec<u32> = requests.iter().map(|r| r.track).collect();
        tracks.sort_unstable(); // Sort Vec
        tracks.dedup(); // Remove consecutive duplicates

        match self.direction {
            CScanDirection::RIGHT => {
                // Split into two parts: from current position to end, and from start to current position

                let (right_part, left_part): (Vec<u32>, Vec<u32>) = tracks
                    .into_iter()
                    .partition(|&track| track >= initial_position);

                // Process right part (current position to max track)
                for &track in &right_part {
                    let seek_distance = current_position.abs_diff(track);
                    total_seek_time += seek_distance;
                    result.seek_sequence.push(track);
                    current_position = track;
                }

                // Prcess left part (0 to current position) after jumping
                if !left_part.is_empty() {
                    if current_position < config.max_track {
                        total_seek_time += current_position.abs_diff(config.max_track);
                    }

                    current_position = 0; // Jump to 0 (no seek time)

                    for &track in &left_part {
                        let seek_distance = current_position.abs_diff(track);
                        total_seek_time += seek_distance;
                        result.seek_sequence.push(track);
                        current_position = track;
                    }
                }
            }
            CScanDirection::LEFT => {
                let (left_part, right_part): (Vec<u32>, Vec<u32>) = tracks
                    .into_iter()
                    .partition(|&track| track <= initial_position);

                // Process left part in descending order (current position to 0)
                let mut left_part_sorted = left_part;
                left_part_sorted.sort_unstable();
                left_part_sorted.reverse(); // Descending Order

                for &track in &left_part_sorted {
                    let seek_distance = current_position.abs_diff(track);
                    total_seek_time += seek_distance;
                    result.seek_sequence.push(track);
                    current_position = track;
                }

                // Process right part (max to current position) after jumping
                if !right_part.is_empty() {
                    if current_position > 0 {
                        total_seek_time += current_position;
                    }

                    // Jump to max (no seek time)
                    current_position = config.max_track;

                    let mut right_part_sorted = right_part;
                    right_part_sorted.sort_unstable();
                    right_part_sorted.reverse(); // Descending order

                    for &track in &right_part_sorted {
                        let seek_distance = current_position.abs_diff(track);
                        total_seek_time += seek_distance;
                        result.seek_sequence.push(track);
                        current_position = track;
                    }
                }
            }
        }

        result.total_seek_time = total_seek_time;
        result.requests_processed = requests.len();
        result.average_seek_time = total_seek_time as f64 / requests.len() as f64;

        result
    }

    fn name(&self) -> String {
        format!("C-SCAN (Direction: {:?})", self.direction)
    }
}