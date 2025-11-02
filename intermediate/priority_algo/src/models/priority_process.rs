#[derive(Debug, Clone)]
pub struct Process {
    pub pid: usize,
    pub arrival_time: u32,
    pub burst_time: u32,
    pub priority: u32,
    pub completion_time: u32,
    pub turnaround_time: u32,
    pub waiting_time: u32,
}

impl Process {
    pub fn new(pid: usize, arrival_time: u32, burst_time: u32) -> Self {
        Process {
            pid,
            arrival_time,
            burst_time,
            priority: 0,
            completion_time: 0,
            turnaround_time: 0,
            waiting_time: 0,
        }
    }
}
