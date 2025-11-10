use crate::{models::cpu_process::HRRNProcess, utils::gantt_display::GanttChart};

pub trait HRRNSched {
    fn schedule(&mut self);
    fn display(&self);
}

pub struct HRRNScheduler {
    pub processes: Vec<HRRNProcess>,
    pub gantt_chart: GanttChart
}

impl HRRNScheduler {
    pub fn new(processes: Vec<HRRNProcess>) -> Self {
        HRRNScheduler { 
            processes, 
            gantt_chart: GanttChart::new()
        }
    }

    pub fn calculate_response_ratio(
        &self,
        process_index: usize,
        current_time: u32
    ) -> f32 {
        let process: &HRRNProcess = &self.processes[process_index];
        let waiting_time = current_time.saturating_sub(process.base.arrival_time);

        // Response Ration Formula
        (waiting_time as f32 + process.base.burst_time as f32) / process.base.burst_time as f32
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

