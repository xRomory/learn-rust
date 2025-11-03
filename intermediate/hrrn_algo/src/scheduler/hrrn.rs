use crate::{
    models::hrrn_process::Process,
    utils::gantt_display::{GanttChart, GanttSegment},
};

pub trait Scheduler {
    fn scheduler(&mut self);
    fn display(&self);
}

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
}

impl Scheduler for HRRNScheduler {
    fn scheduler(&mut self) {
        self.processes.sort_by_key(|p| p.arrival_time);

        let num_processes = self.processes.len();
        let mut current_time = 0;
        let mut completed_process = 0;
        let mut is_completed = vec![false; num_processes];
        let mut gantt_chart = GanttChart::new();

        while completed_process < num_processes {
            let mut idx: Option<usize> = None;
            let mut highest_rr = f64::MIN;

            for (i, p) in self.processes.iter().enumerate() {
                if p.arrival_time <= current_time && !is_completed[i] {
                    let response_ratio = self.processes[i].calculate_response_ratio(current_time);

                    if response_ratio > highest_rr {
                        highest_rr = response_ratio;
                        idx = Some(i);
                    }
                }
            }

            match idx {
                Some(i) => {
                    let process = &mut self.processes[i];
                    let start_time = current_time;
                    let end_time = start_time + process.burst_time;

                    process.completion_time = end_time;
                    process.turnaround_time = end_time - process.arrival_time;
                    process.waiting_time = process.turnaround_time - process.burst_time;
                    process.response_time = (start_time as u32) - (process.arrival_time as u32);

                    gantt_chart.segments.push(GanttSegment {
                        pid: process.pid,
                        start_time,
                        end_time,
                    });

                    is_completed[i] = true;
                    completed_process += 1;
                    current_time = end_time;
                }
                None => {
                    current_time += 1;
                }
            }
        }

        self.gantt_chart = gantt_chart;

        let total_tat: u32 = self.processes.iter().map(|p| p.turnaround_time).sum();
        let total_wt: u32 = self.processes.iter().map(|p| p.waiting_time).sum();

        self.avg_tat = total_tat as f64 / num_processes as f64;
        self.avg_wt = total_wt as f64 / num_processes as f64;
    }

    fn display(&self) {
        println!("\n|---------|----|----|----|-----|----|----|");
        println!(
            "|{:^9}|{:^4}|{:^4}|{:^4}|{:^5}|{:^4}|{:^4}|",
            "Process", "AT", "BT", "CT", "TAT", "WT", "RT"
        );
        println!("|---------|----|----|----|-----|----|----|");

        for p in &self.processes {
            println!(
                "|{:^9}|{:^4}|{:^4}|{:^4}|{:^5}|{:^4}|{:^4}|",
                p.pid,
                p.arrival_time,
                p.burst_time,
                p.completion_time,
                p.turnaround_time,
                p.waiting_time,
                p.response_time,
            );
        }

        println!("|---------|----|----|----|-----|----|----|");
        println!("Average Turnaround Time: {:.0}ms", self.avg_tat);
        println!("Average Waiting Time: {:.0}ms", self.avg_wt);

        self.gantt_chart.display_gantt_chart();
    }
}
