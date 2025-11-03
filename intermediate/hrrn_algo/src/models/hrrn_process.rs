pub struct Process {
    pub pid: usize,
    pub arrival_time: u32,
    pub burst_time: u32,
    pub completion_time: u32,
    pub response_time: u32,
    pub turnaround_time: u32,
    pub waiting_time: u32,
}

impl Process {
    pub fn new(pid: usize, arrival_time: u32, burst_time: u32) -> Self {
        Process {
            pid,
            arrival_time,
            burst_time,
            completion_time: 0,
            response_time: 0,
            turnaround_time: 0,
            waiting_time: 0,
        }
    }

    pub fn calculate_response_ratio(&self, current_time: u32) -> f64 {
        let waiting_time = if current_time > self.arrival_time {
            current_time - self.arrival_time
        } else {
            0
        };

        (waiting_time as f64 + self.burst_time as f64) / self.burst_time as f64
    }
}
