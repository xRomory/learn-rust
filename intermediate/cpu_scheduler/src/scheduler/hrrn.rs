use crate::{models::cpu_process::HRRNProcess, utils::gantt_display::{GanttChart, GanttSegment}};

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

    fn display_table(processes: &[HRRNProcess]) {
        const HEADER: &str = "|---------|----|----|----|-----|----|----|";

        println!("\n{}", HEADER);
        println!(
            "|{:^9}|{:^4}|{:^4}|{:^4}|{:^5}|{:^4}|{:^4}|",
            "Process", "AT", "BT", "CT", "TAT", "WT", "RT"
        );
        println!("{}", HEADER);

        for p in processes {
            println!(
                "|{:^9}|{:^4}|{:^4}|{:^4}|{:^5}|{:^4}|{:^4}|",
                p.base.pid,
                p.base.arrival_time,
                p.base.burst_time,
                p.completion_time,
                p.turnaround_time,
                p.waiting_time,
                p.response_time,
            );
        }

        println!("{}\n", HEADER);
    }
}

impl HRRNSched for HRRNScheduler {
    fn schedule(&mut self) {
        self.processes.sort_by_key(|p: &HRRNProcess| p.base.arrival_time);

        let num_processes = self.processes.len();
        let mut completed_process = 0;
        let mut current_time = 0;
        let mut is_completed = vec![false; num_processes];
        let mut next_arrival_index = 0;

        while completed_process < num_processes {
            let mut selected_index = None;
            let mut highest_response_ratio = f32::MIN;

            for i in next_arrival_index..num_processes {
                if self.processes[i].base.arrival_time > current_time {
                    break;
                }

                if !is_completed[i] {
                    let response_ratio = self.calculate_response_ratio(i, current_time);

                    if response_ratio > highest_response_ratio {
                        highest_response_ratio = response_ratio;
                        selected_index = Some(i);
                    }
                }
            }

            match selected_index {
                Some(i) => {
                    let process = &mut self.processes[i];
                    let start_time = current_time;
                    let end_time = start_time + process.base.burst_time;

                    process.completion_time = end_time;
                    process.turnaround_time = end_time - process.base.arrival_time;
                    process.waiting_time = process.turnaround_time - process.base.burst_time;
                    process.response_time = start_time.saturating_sub(process.base.arrival_time);

                    self.gantt_chart.segments.push(GanttSegment {
                        pid: process.base.pid,
                        start_time,
                        end_time
                    });

                    is_completed[i] = true;
                    completed_process += 1;
                    current_time = end_time;

                    while next_arrival_index < num_processes
                        && self.processes[next_arrival_index].base.arrival_time <= current_time
                    {
                        next_arrival_index += 1;
                    }
                }
                None => {
                    if next_arrival_index < num_processes {
                        current_time = self.processes[next_arrival_index].base.arrival_time;
                    } else {
                        current_time += 1;
                    }
                }
            }
        }
    }

    fn display(&self) {
        Self::display_table(&self.processes);

        println!("Average Turnaround Time: {:.0}ms", self.avg_turnaround_time());
        println!("Average Waiting Time: {:.0}ms", self.avg_waiting_time());

        self.gantt_chart.display_gantt_chart();
    }
}