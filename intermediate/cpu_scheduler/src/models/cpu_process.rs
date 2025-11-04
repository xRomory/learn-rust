#[derive(Debug, Clone)]
pub struct FCFSProcess {
    pub pid: usize,
    pub arrival_time: u32,
    pub burst_time: u32,
    pub start_time: u32,
    pub completion_time: u32,
    pub turnaround_time: u32,
    pub waiting_time: u32,
}

impl FCFSProcess {
    pub fn new(pid: usize, arrival_time: u32, burst_time: u32) -> Self {
        FCFSProcess {
            pid,
            arrival_time,
            burst_time,
            start_time: 0,
            completion_time: 0,
            turnaround_time: 0,
            waiting_time: 0,
        }
    }
}
