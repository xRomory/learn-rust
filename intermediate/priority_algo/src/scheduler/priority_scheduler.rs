use crate::models::priority_process::Process;

pub trait Scheduler {
    fn schedule(&mut self);
    fn display(&self);
}

pub struct PriorityScheduler {
    pub processes: Vec<Process>,
    pub avg_tat: f32,
    pub avg_wt: f32,
}
