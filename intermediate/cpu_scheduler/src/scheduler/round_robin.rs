use std::{cmp::min, collections::VecDeque};

use crate::{
    models::cpu_process::RRProcess,
    utils::gantt_display::{GanttChart, GanttSegment},
};

pub trait RRScheduler {
    fn schedule(&mut self);
    fn display(&self);
}

pub struct RoundRobinAlgorithm {
    pub processes: Vec<RRProcess>,
    pub gantt_chart: GanttChart,
    pub time_quantum: u32,
    pub avg_tat: f32,
    pub avg_wt: f32,
}

impl RoundRobinAlgorithm {
    pub fn new(time_quantum: u32, processes: Vec<RRProcess>) -> Self {
        RoundRobinAlgorithm {
            processes,
            gantt_chart: GanttChart::new(),
            time_quantum,
            avg_tat: 0.0,
            avg_wt: 0.0,
        }
    }
}

impl RRScheduler for RoundRobinAlgorithm {
    fn schedule(&mut self) {
        self.processes.sort_by_key(|p| p.base.arrival_time);

        let num_processes = self.processes.len();
        let mut ready_queue: VecDeque<usize> = VecDeque::new();
        let mut current_time = 0;
        let mut completed_process = 0;
        let mut next_arrival_index = 0;

        while next_arrival_index < num_processes
            && self.processes[next_arrival_index].base.arrival_time <= current_time
        {
            ready_queue.push_back(next_arrival_index);
            next_arrival_index += 1;
        }

        while completed_process < num_processes {
            if ready_queue.is_empty() {
                if next_arrival_index < num_processes {
                    current_time = self.processes[next_arrival_index].base.arrival_time;
                    ready_queue.push_back(next_arrival_index);
                    next_arrival_index += 1;
                }
                continue;
            }

            if let Some(process_index) = ready_queue.pop_front() {
                // Calculate execution time for this time quantum
                let execution_time = min(
                    self.time_quantum,
                    self.processes[process_index].remaining_time,
                );
                let pid = self.processes[process_index].base.pid;
                let start_time = current_time;

                self.processes[process_index].remaining_time -= execution_time;
                current_time += execution_time;

                self.gantt_chart.segments.push(GanttSegment {
                    pid,
                    start_time,
                    end_time: current_time,
                });

                while next_arrival_index < num_processes
                    && self.processes[next_arrival_index].base.arrival_time <= current_time 
                {
                    ready_queue.push_back(next_arrival_index);
                    next_arrival_index += 1;
                }

                if self.processes[process_index].remaining_time == 0 {
                    let process = &mut self.processes[process_index];
                    process.completion_time = current_time;
                    process.turnaround_time = process.completion_time - process.base.arrival_time;
                    process.waiting_time = process.turnaround_time - process.base.burst_time;
                    completed_process += 1;
                } else {
                    ready_queue.push_back(process_index);
                }
            }
        }

        let total_tat: u32 = self.processes.iter().map(|t| t.turnaround_time).sum();
        let total_wt: u32 = self.processes.iter().map(|t| t.waiting_time).sum();

        self.avg_tat = total_tat as f32 / num_processes as f32;
        self.avg_wt = total_wt as f32 / num_processes as f32;
    }

    fn display(&self) {
        println!("|---------|----|----|----|----|-----|");
        println!(
            "|{:^9}|{:^4}|{:^4}|{:^4}|{:^4}|{:^5}|",
            "Process", "AT", "BT", "CT", "TAT", "WT",
        );
        println!("|---------|----|----|----|----|-----|");

        for p in &self.processes {
            println!(
                "|{:^9}|{:^4}|{:^4}|{:^4}|{:^4}|{:^5}|",
                p.base.pid,
                p.base.arrival_time,
                p.base.burst_time,
                p.completion_time,
                p.turnaround_time,
                p.waiting_time,
            );
        }

        println!("|---------|----|----|----|----|-----|");
        println!("\nAverage Turnaround Time: {:.2}", self.avg_tat);
        println!("\nAverage Waiting Time: {:.2}", self.avg_wt);

        self.gantt_chart.display_gantt_chart();
    }
}
