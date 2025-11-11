#[derive(Debug, Clone, Copy)]
pub struct DiskRequest {
    pub request_id: usize,
    pub track_number: u32,
    pub arrival_time: u32,
}

impl DiskRequest {
    pub fn new(request_id: usize, track_number: u32) -> Self {
        DiskRequest { 
            request_id,
            track_number,
            arrival_time: 0,
        }
    }
}