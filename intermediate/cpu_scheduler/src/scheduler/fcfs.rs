use crate::models::cpu_process::FCFSProcess;

pub trait Scheduler {
    fn scheduler(&mut self);
    fn display(&self);
}

pub struct FCFSScheduler {
    pub processes: Vec<FCFSProcess>,
    pub avg_tat: f32,
    pub avg_wt: f32,
}

impl FCFSScheduler {
    fn new(processes: Vec<FCFSProcess>) -> Self {
        Self {
            processes,
            avg_tat: 0.0,
            avg_wt: 0.0,
        }
    }
}

impl Scheduler for FCFSScheduler {
    fn scheduler(&mut self) {
        self.processes.sort_by_key(|p| p.arrival_time);
    }

    fn display(&self) {}
}
