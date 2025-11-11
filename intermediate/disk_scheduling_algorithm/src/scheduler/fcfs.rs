use crate::models::disk_request::DiskRequest;

pub struct FCFSScheduler {
    pub requests: Vec<DiskRequest>,
    pub head_start: u32,
    pub seek_sequence: Vec<u32>,
    pub total_head_movement: u32,
    pub average_seek_time: f64
}

impl FCFSScheduler {
    pub fn new(requests: Vec<DiskRequest>, head_start: u32) -> Self {
        FCFSScheduler { 
            requests,
            head_start,
            seek_sequence: Vec::new(),
            total_head_movement: 0,
            average_seek_time: 0.0
        }
    }
}