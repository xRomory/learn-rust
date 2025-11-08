use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{
    models::cpu_process::PriorityProcess, 
    utils::gantt_display::{GanttChart, GanttSegment}
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

    // Helper function to add all newly arrived processes to the ready queue
    fn add_arrived_processes_to_heap(
        &self,
        heap: &mut BinaryHeap<Reverse<(u32, u32, usize, usize)>>,
        next_arrival: &mut usize,
        current_time: u32
    ) {
        while *next_arrival < self.processes.len()
            && self.processes[*next_arrival].base.arrival_time <= current_time
        {
            let process = &self.processes[*next_arrival];
            heap.push(Reverse((
                process.priority,
                process.base.arrival_time,
                process.base.pid,
                *next_arrival,
            )));

            *next_arrival += 1;
        }
    }

    fn display_table(processes: &[PriorityProcess]) {
        const HEADER: &str = "|---------|----|----|----------|----|-----|-----|";

        println!("{}", HEADER);
        println!(
            "|{:^9}|{:^4}|{:^4}|{:^10}|{:^4}|{:^5}|{:^5}|",
            "Process", "AT", "BT", "Priority", "CT", "TAT", "WT"
        );
        println!("{}", HEADER);

        for p in processes {
            println!(
                "|{:^9}|{:^4}|{:^4}|{:^10}|{:^4}|{:^5}|{:^5}|",
                p.base.pid,
                p.base.arrival_time,
                p.base.burst_time,
                p.priority,
                p.completion_time,
                p.turnaround_time,
                p.waiting_time,
            );
        }

        println!("{}", HEADER);
    }
}

impl PrioritySched for PriorityScheduler {
    fn schedule(&mut self) {
        self.processes.sort_by_key(|p| p.base.arrival_time);

        let num_process = self.processes.len();
        let mut completed_process = 0;
        let mut current_time = 0;
        let mut next_arrival = 0;

        let mut heap = BinaryHeap::new();

        while completed_process < num_process {
            self.add_arrived_processes_to_heap(
                &mut heap,
                &mut next_arrival, 
                current_time
            );

            if let Some(Reverse((
                _priority,
                _arrival,
                _pid,
                idx
            ))) = heap.pop() {
                let process = &mut self.processes[idx];

                let start_time = current_time;
                let end_time = start_time + process.base.burst_time;

                process.completion_time = end_time;
                process.turnaround_time = end_time - process.base.arrival_time;
                process.waiting_time = process.turnaround_time - process.base.burst_time;

                self.gantt_chart.segments.push(GanttSegment {
                    pid: process.base.pid,
                    start_time,
                    end_time
                });

                current_time = end_time;
                completed_process += 1;

                self.add_arrived_processes_to_heap(
                    &mut heap,
                    &mut next_arrival, 
                    current_time
                );
            } else {
                if next_arrival < num_process { current_time = self.processes[next_arrival].base.arrival_time; }
            }
        }
    }

    fn display(&self) {
        Self::display_table(&self.processes);
        println!("Average Turnaround Time: {:.2}", self.avg_turnaround_time());
        println!("Average Waiting Time: {:.2}", self.avg_waiting_time());

        self.gantt_chart.display_gantt_chart();
    }
}