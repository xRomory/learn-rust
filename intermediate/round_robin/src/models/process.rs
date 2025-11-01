#[derive(Debug, Clone)]
pub struct Process {
    pub pid: usize,
    pub arrival_time: i32,
    pub burst_time: i32,
    pub remaining_time: i32,
    pub completion_time: i32,
    pub turnaround_time: i32,
    pub waiting_time: i32,
}

impl Process {
    pub fn new(pid: usize, arrival_time: i32, burst_time: i32) -> Self {
        Process {
            pid,
            arrival_time,
            burst_time,
            remaining_time: burst_time,
            completion_time: 0,
            turnaround_time: 0,
            waiting_time: 0,
        }
    }
}
