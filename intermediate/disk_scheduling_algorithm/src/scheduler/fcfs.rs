use crate::models::{disk_request::DiskRequest, disk_schedule_result::{DiskScheduleResult, ScheduleMetrics}};

pub trait DiskScheduler {
    fn schedule(&mut self) -> Vec<DiskScheduleResult>;
    fn display(&self) -> &[DiskScheduleResult];
    fn calculate_metrics(&self, results: &[DiskScheduleResult]) -> ScheduleMetrics;
}

pub struct FCFSScheduler {
    pub requests: Vec<DiskRequest>,
    pub initial_head_position: u32,
    pub total_tracks: u32,
}

impl FCFSScheduler {
    pub fn new(
        requests: Vec<DiskRequest>,
        initial_head_position: u32,
        total_tracks: u32,
    ) -> Self {
        Self { 
            requests,
            initial_head_position,
            total_tracks,
        }
    }
}

impl DiskScheduler for FCFSScheduler {
    fn schedule(&mut self) -> Vec<DiskScheduleResult> {
        let mut results = Vec::new();
        let mut current_head = self.initial_head_position;
        let mut current_time = 0;

        for (sequence, request) in self.requests.iter().enumerate() {
            let seek_time = request.track_number.abs_diff(current_head);
            let service_time = 1;

            let result = DiskScheduleResult {
                request_id: request.request_id,
                track_number: request.track_number,
                start_time: current_time,
                end_time: current_time + service_time,
                seek_time,
            };

            results.push(result);

            current_head = request.track_number;
            current_time += service_time;
        }

        results
    }

    fn display(&self) -> &[DiskScheduleResult] {
        
    }

    fn calculate_metrics(&self, results: &[DiskScheduleResult]) -> ScheduleMetrics {
        
    }
}