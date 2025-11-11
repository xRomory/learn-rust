#[derive(Debug, Clone)]
pub struct DiskScheduleResult {
    pub request_id: usize,
    pub track_number: u32,
    pub start_time: u32,
    pub end_time: u32,
    pub seek_time: u32
}

#[derive(Debug)]
pub struct ScheduleMetrics {
    pub total_seek_time: u32,
    pub average_seek_time: f64,
    pub throughput: f64,
    pub total_time: u32
}