#[derive(Debug, Clone)]
pub struct BaseProcess {
    pub pid: usize,
    pub arrival_time: u32,
    pub burst_time: u32,
}

impl BaseProcess {
    pub fn new(pid: usize, arrival_time: u32, burst_time: u32) -> Self {
        BaseProcess {
            pid,
            arrival_time,
            burst_time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FCFSProcess {
    pub base: BaseProcess,
    pub start_time: u32,
    pub completion_time: u32,
    pub turnaround_time: u32,
    pub waiting_time: u32,
}

impl FCFSProcess {
    pub fn new(base: BaseProcess) -> Self {
        FCFSProcess {
            base,
            start_time: 0,
            completion_time: 0,
            turnaround_time: 0,
            waiting_time: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SJFProcess {
    pub base: BaseProcess,
    pub remaining_time: u32,
    pub completion_time: u32,
    pub turnaround_time: u32,
    pub waiting_time: u32,
}

impl SJFProcess {
    pub fn new(base: BaseProcess) -> Self {
        SJFProcess {
            remaining_time: base.burst_time,
            base,
            completion_time: 0,
            turnaround_time: 0,
            waiting_time: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RRProcess {
    pub base: BaseProcess,
    pub remaining_time: u32,
    pub completion_time: u32,
    pub turnaround_time: u32,
    pub waiting_time: u32,
}

impl RRProcess {
    pub fn new(base: BaseProcess) -> Self {
        RRProcess {
            remaining_time: base.burst_time,
            base,
            completion_time: 0,
            turnaround_time: 0,
            waiting_time: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PriorityProcess {
    pub base: BaseProcess,
    pub priority: u32,
    pub completion_time: u32,
    pub turnaround_time: u32,
    pub waiting_time: u32
}

impl PriorityProcess {
    pub fn new(base: BaseProcess) -> Self {
        PriorityProcess { 
            base,
            priority: 0,
            completion_time: 0,
            turnaround_time: 0,
            waiting_time: 0 
        }
    }
}