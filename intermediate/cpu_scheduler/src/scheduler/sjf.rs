use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{
    models::cpu_process::SJFProcess,
    utils::gantt_display::{GanttChart, GanttSegment},
};

pub trait SJFScheduler {
    fn schedule(&mut self);
    fn display(&self);
}

pub struct SJFPreemptiveScheduler {
    pub processes: Vec<SJFProcess>,
    pub gantt_chart: GanttChart,
    pub avg_tat: f32,
    pub avg_wt: f32,
}

impl SJFPreemptiveScheduler {
    pub fn new(processes: Vec<SJFProcess>) -> Self {
        Self {
            processes,
            avg_tat: 0.0,
            avg_wt: 0.0,
            gantt_chart: GanttChart::new(),
        }
    }
}

impl SJFScheduler for SJFPreemptiveScheduler {
    fn schedule(&mut self) {
        self.processes.sort_by_key(|p| p.base.arrival_time);

        let num_of_processes = self.processes.len();
        let mut time = 0;
        let mut completed_process = 0;

        let mut heap = BinaryHeap::new();
        let mut next_proc = 0;

        while completed_process < num_of_processes {
            while next_proc < num_of_processes
                && self.processes[next_proc].base.arrival_time <= time
            {
                heap.push(Reverse((
                    self.processes[next_proc].remaining_time,
                    self.processes[next_proc].base.arrival_time,
                    next_proc,
                )));
                next_proc += 1;
            }

            if let Some(Reverse((_, _, idx))) = heap.pop() {
                let process = &mut self.processes[idx];

                // Execute for one unit of time
                process.remaining_time -= 1;
                self.gantt_chart.segments.push(GanttSegment {
                    pid: process.base.pid,
                    start_time: time,
                    end_time: time + 1,
                });

                time += 1;

                if process.remaining_time == 0 {
                    process.completion_time = time;
                    process.turnaround_time = time - process.base.arrival_time;
                    process.waiting_time = process.turnaround_time - process.base.burst_time;
                    completed_process += 1;
                } else {
                    // Push it back into heap with updated remaining time
                    heap.push(Reverse((
                        process.remaining_time,
                        process.base.arrival_time,
                        idx,
                    )));
                }
            } else {
                // No process is ready; jump to the next arrival
                if next_proc < num_of_processes {
                    time = self.processes[next_proc].base.arrival_time;
                }
            }
        }

        let total_tat: u32 = self.processes.iter().map(|p| p.turnaround_time).sum();
        let total_wt: u32 = self.processes.iter().map(|p| p.waiting_time).sum();

        self.avg_tat = total_tat as f32 / num_of_processes as f32;
        self.avg_wt = total_wt as f32 / num_of_processes as f32;
    }

    fn display(&self) {
        println!("|---------|----|----|----|----|-----|----|");
        println!(
            "|{:^9}|{:^4}|{:^4}|{:^4}|{:^4}|{:^5}|{:^4}|",
            "Process", "AT", "BT", "CT", "TAT", "WT", ""
        );
        println!("|---------|----|----|----|----|-----|----|");

        for p in &self.processes {
            println!(
                "|{:^9}|{:^4}|{:^4}|{:^4}|{:^4}|{:^5}|{:^4}|",
                p.base.pid,
                p.base.arrival_time,
                p.base.burst_time,
                p.completion_time,
                p.turnaround_time,
                p.waiting_time,
                "",
            )
        }

        println!("|---------|----|----|----|----|-----|----|");
        println!("\nAverage Turnaround Time: {:.2}", self.avg_tat);
        println!("\nAverage Waiting Time: {:.2}", self.avg_wt);

        self.gantt_chart.display_gantt_chart();
    }
}
