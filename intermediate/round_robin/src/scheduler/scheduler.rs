use crate::models::process::Process;
use std::collections::VecDeque;

pub trait Scheduler {
    fn schedule(&mut self);
    fn display(self);
}

pub struct RoundRobinScheduler {
    pub processes: VecDeque<Process>,
    pub time_quantum: i32,
    pub avg_tat: f32,
    pub avg_wt: f32,
}
