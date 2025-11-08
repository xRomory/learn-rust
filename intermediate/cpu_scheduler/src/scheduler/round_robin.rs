use std::{cmp::min, collections::VecDeque};

use crate::{
    models::cpu_process::RRProcess,
    utils::gantt_display::{GanttChart, GanttSegment},
};

pub trait RRScheduler {
    fn schedule(&mut self);
    fn display(&self);
}

pub struct RoundRobinScheduler {
    pub processes: Vec<RRProcess>,
    pub gantt_chart: GanttChart,
    pub time_quantum: u32,
}

impl RoundRobinScheduler {
    pub fn new(time_quantum: u32, processes: Vec<RRProcess>) -> Self {
        RoundRobinScheduler {
            processes,
            gantt_chart: GanttChart::new(),
            time_quantum,
        }
    }

    pub fn avg_turnaround_time(&self) -> f32 {
        self.processes
            .iter()
            .map(|t| t.turnaround_time as f32)
            .sum::<f32>() / self.processes.len() as f32
    }

    pub fn avg_waiting_time(&self) -> f32 {
        self.processes
            .iter()
            .map(|t| t.waiting_time as f32)
            .sum::<f32>() / self.processes.len() as f32
    }

    // Helper function to add all newly arrived processes to the ready queue
    fn add_arrived_processes(
        &self,
        ready_queue: &mut VecDeque<usize>,
        next_arrival_index: &mut usize,
        current_time: u32,
    ) {
        while *next_arrival_index < self.processes.len()
            && self.processes[*next_arrival_index].base.arrival_time <= current_time
        {
            ready_queue.push_back(*next_arrival_index);
            *next_arrival_index += 1;
        }
    }

    // Helper for Table display
    fn display_table(processes: &[RRProcess]) {
        const HEADER: &str = "|---------|----|----|----|----|-----|";

        println!("{}", HEADER);
        println!(
            "|{:^9}|{:^4}|{:^4}|{:^4}|{:^4}|{:^5}|",
            "Process", "AT", "BT", "CT", "TAT", "WT",
        );
        println!("{}", HEADER);

        for p in processes {
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

        println!("{}", HEADER);
    }
}

impl RRScheduler for RoundRobinScheduler {
    fn schedule(&mut self) {
        self.processes.sort_by_key(|p| p.base.arrival_time);

        let num_processes = self.processes.len();
        let mut ready_queue: VecDeque<usize> = VecDeque::new();
        let mut current_time = 0;
        let mut completed_process = 0;
        let mut next_arrival_index = 0;

        self.add_arrived_processes(
            &mut ready_queue,
            &mut next_arrival_index, 
            current_time
        );

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

                let start_time = current_time;

                self.processes[process_index].remaining_time -= execution_time;
                current_time += execution_time;

                self.gantt_chart.segments.push(GanttSegment {
                    pid: self.processes[process_index].base.pid,
                    start_time,
                    end_time: current_time,
                });

                self.add_arrived_processes(
                    &mut ready_queue,
                    &mut next_arrival_index, 
                    current_time
                );

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
    }

    fn display(&self) {
        Self::display_table(&self.processes);
        println!("\nAverage Turnaround Time: {:.2}", self.avg_turnaround_time());
        println!("\nAverage Waiting Time: {:.2}", self.avg_waiting_time());

        self.gantt_chart.display_gantt_chart();
    }
}
