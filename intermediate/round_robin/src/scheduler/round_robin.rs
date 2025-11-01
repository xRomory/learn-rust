use std::{
    collections::VecDeque,
    cmp::min
};
use crate::models::rr_process::Process;

pub trait Scheduler {
    fn schedule(&mut self);
    fn display(&self);
    fn add_process(&mut self, process: Process);
}

#[derive(Debug)]
pub struct GanttSegment {
    pid: usize,
    start_time: u32,
    end_time: u32,
}

pub struct RoundRobinScheduler {
    pub processes: Vec<Process>,
    pub time_quantum: u32,
    pub gantt_chart: Vec<GanttSegment>,
}

impl RoundRobinScheduler {
    pub fn new(time_quantum: u32) -> Self {
        RoundRobinScheduler {
            processes: Vec::new(),
            time_quantum,
            gantt_chart: Vec::new(),
        }
    }

    fn display_gantt_chart(&self) {
        if self.gantt_chart.is_empty() {
            return;
        }

        println!("\n{:=^80}", " GANTT CHART ");
        
        // Print top border
        print!("\n");
        for segment in &self.gantt_chart {
            let width = (segment.end_time - segment.start_time) as usize * 4;
            print!("+{}", "-".repeat(width));
        }
        println!("+");

        // Print process IDs
        print!("|");
        for segment in &self.gantt_chart {
            let width = (segment.end_time - segment.start_time) as usize * 4;
            let label = format!("P{}", segment.pid);
            let padding = (width - label.len()) / 2;
            print!("{}{}{}", " ".repeat(padding), label, " ".repeat(width - padding - label.len()));
            print!("|");
        }
        println!();

        // Print bottom border
        for segment in &self.gantt_chart {
            let width = (segment.end_time - segment.start_time) as usize * 4;
            print!("+{}", "-".repeat(width));
        }
        println!("+");

        // Print time labels
        print!("{}", self.gantt_chart[0].start_time);
        for segment in &self.gantt_chart {
            let width = (segment.end_time - segment.start_time) as usize * 4;
            let time_label = segment.end_time.to_string();
            print!("{}{}", " ".repeat(width - time_label.len() + 1), time_label);
        }
        println!("\n");
    }
}

impl Scheduler for RoundRobinScheduler {
    fn add_process(&mut self, process: Process) {
        self.processes.push(process);
    }

    fn schedule(&mut self) {
        self.processes.sort_by_key(|p| p.arrival_time);

        let mut ready_queue: VecDeque<usize> = VecDeque::new();
        let mut current_time = 0;
        let mut completed_process = 0;
        let num_processes = self.processes.len();
        let mut gantt_chart = Vec::new();
        let mut next_arrival_index = 0;

        // Add process that have arrived at time 0
        while next_arrival_index < num_processes
            && self.processes[next_arrival_index].arrival_time <= current_time
        {
            ready_queue.push_back(next_arrival_index);
            next_arrival_index += 1;
        }

        while completed_process < num_processes {
            if ready_queue.is_empty() {
                if next_arrival_index < num_processes {
                    current_time = self.processes[next_arrival_index].arrival_time;
                    ready_queue.push_back(next_arrival_index);
                    next_arrival_index += 1;
                }
                continue;
            }

            if let Some(process_index) = ready_queue.pop_front() {
                // Calculate execution time for this quantum
                let execution_time = min(
                    self.time_quantum,
                    self.processes[process_index].remaining_time
                );
                let pid = self.processes[process_index].pid;
                let start_time = current_time;

                // Execute the process
                self.processes[process_index].remaining_time -= execution_time;
                current_time += execution_time;

                gantt_chart.push(GanttSegment {
                    pid,
                    start_time,
                    end_time: current_time
                });

                while next_arrival_index < num_processes 
                    && self.processes[next_arrival_index].arrival_time <= current_time 
                {
                    ready_queue.push_back(next_arrival_index);
                    next_arrival_index += 1;
                }

                if self.processes[process_index].remaining_time == 0 {
                    let process = &mut self.processes[process_index];
                    process.completion_time = current_time;
                    process.turnaround_time = process.completion_time - process.arrival_time;
                    process.waiting_time = process.turnaround_time - process.burst_time;
                    completed_process += 1;
                } else {
                    ready_queue.push_back(process_index);
                }
            }
        }

        self.gantt_chart = gantt_chart;
    }

    fn display(&self) {
        println!("|---------|----|----|----|----|-----|----|");
        println!(
            "|{:^9}|{:^4}|{:^4}|{:^4}|{:^4}|{:^5}|{:^4}|",
            "Process", "AT", "BT", "CT", "TAT", "WT", ""
        );
        println!("|---------|----|----|----|----|-----|----|");

        let mut total_tat = 0;
        let mut total_wt = 0;

        for p in &self.processes {
            println!(
                "|{:^9}|{:^4}|{:^4}|{:^4}|{:^4}|{:^5}|{:^4}|",
                p.pid,
                p.arrival_time,
                p.burst_time,
                p.completion_time,
                p.turnaround_time,
                p.waiting_time,
                ""
            );

            total_tat += p.turnaround_time;
            total_wt += p.waiting_time;
        }

        println!("|---------|----|----|----|----|-----|----|");

         // Display averages
        let n = self.processes.len() as f64;
        println!("\n{:=^80}", " AVERAGES ");
        println!("Average Turnaround Time: {:.2}", total_tat as f64 / n);
        println!("Average Waiting Time: {:.2}", total_wt as f64 / n);

        self.display_gantt_chart();
    }
}
