use crate::{models::hrrn_process::Process, utils::gantt_display::GanttChart};

pub struct HRRNScheduler {
    pub processes: Vec<Process>,
    pub gantt_chart: GanttChart,
    pub avg_tat: f64,
    pub avg_wt: f64,
}

impl HRRNScheduler {
    pub fn new(processes: Vec<Process>) -> Self {
        HRRNScheduler {
            processes,
            avg_tat: 0.0,
            avg_wt: 0.0,
            gantt_chart: GanttChart::new(),
        }
    }

    pub fn scheduler(&mut self) {
        self.processes.sort_by_key(|p| p.arrival_time);

        let num_processes = self.processes.len();
        let mut current_time = 0;
        let mut completed_process = 0;
        let mut is_completed = vec![false; num_processes];
        let mut gantt_chart = Vec::new();

        while completed_process < num_processes {
            let mut idx: Option<usize> = None;
            let mut highest_rr = f64::MIN;

            for (i, p) in self.processes.iter().enumerate() {
                if p.arrival_time <= current_time && !is_completed[i] {
                    let waiting_time = current_time - p.arrival_time;
                    let response_ratio = self.processes[i].calculate_response_ratio(current_time);

                    if response_ratio > highest_rr {
                        highest_rr = response_ratio;
                        idx = Some(i);
                    }
                }
            }
        }
    }
}
