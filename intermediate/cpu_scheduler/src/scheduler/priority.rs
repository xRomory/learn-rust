use crate::{
    models::cpu_process::PriorityProcess, 
    utils::gantt_display::{GanttChart}
};

pub trait PrioritySched {
    fn schedule(&mut self);
    fn display(&self);
}

pub struct PriorityScheduler {
    pub processes: Vec<PriorityProcess>,
    pub gantt_chart: GanttChart
}

impl PriorityScheduler {
    pub fn new(processes: Vec<PriorityProcess>) -> Self {
        PriorityScheduler { 
            processes, 
            gantt_chart: GanttChart::new() 
        }
    }

    pub fn avg_turnaround_time(&self) -> f32 {
        self.processes
            .iter()
            .map(|p| p.turnaround_time as f32)
            .sum::<f32>() / self.processes.len() as f32
    }

    pub fn avg_waiting_time(&self) -> f32 {
        self.processes
            .iter()
            .map(|p| p.waiting_time as f32)
            .sum::<f32>() / self.processes.len() as f32
    }
}

impl PrioritySched for PriorityScheduler {
    fn schedule(&mut self) {
        self.processes.sort_by_key(|p| p.base.arrival_time);

        let num_process = self.processes.len();

    }

    fn display(&self) {
        
    }
}