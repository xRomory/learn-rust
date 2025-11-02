use crate::models::priority_process::Process;

pub trait Scheduler {
    fn schedule(&mut self);
    fn display(&self);
}

pub struct PriorityScheduler {
    pub processes: Vec<Process>,
    pub avg_tat: f32,
    pub avg_wt: f32,
    gantt_chart: Vec<GanttSegment>,
}

#[derive(Debug)]
struct GanttSegment {
    pid: usize,
    start_time: u32,
    end_time: u32,
}

impl PriorityScheduler {
    pub fn new(processes: Vec<Process>) -> Self {
        PriorityScheduler {
            processes,
            avg_wt: 0.0,
            avg_tat: 0.0,
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

impl Scheduler for PriorityScheduler {
    fn schedule(&mut self) {
        self.processes.sort_by_key(|p| p.arrival_time);

        let num_process = self.processes.len();
        let mut completed_process = 0;
        let mut current_time = 0;
        let mut is_completed = vec![false; num_process];
        let mut gantt_chart = Vec::new();

        while completed_process < num_process {
            let mut idx: Option<usize> = None;
            let mut highest_priority = u32::MAX; //smaller = highest priority

            for (i, p) in self.processes.iter().enumerate() {
                if p.arrival_time <= current_time && !is_completed[i] {
                    if p.priority < highest_priority {
                        highest_priority = p.priority;
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
                    process.turnaround_time = process.completion_time - process.arrival_time;
                    process.waiting_time = process.turnaround_time - process.burst_time;

                    gantt_chart.push(GanttSegment {
                        pid: process.pid,
                        start_time,
                        end_time
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

        // Compute averages:
        let total_tat: u32 = self.processes.iter().map(|p| p.turnaround_time).sum();
        let total_wt: u32 = self.processes.iter().map(|p| p.waiting_time).sum();

        self.avg_tat = total_tat as f32 / num_process as f32;
        self.avg_wt = total_wt as f32 / num_process as f32;
    }

    fn display(&self) {
        println!("|---------|----|----|----------|----|-----|-----|");
        println!(
            "|{:^9}|{:^4}|{:^4}|{:^10}|{:^4}|{:^5}|{:^5}|",
            "Process", "AT", "BT", "Priority", "CT", "TAT", "WT"
        );
        println!("|---------|----|----|----------|----|-----|-----|");

        for p in &self.processes {
            println!(
                "|{:^9}|{:^4}|{:^4}|{:^10}|{:^4}|{:^5}|{:^5}|",
                p.pid,
                p.arrival_time,
                p.burst_time,
                p.priority,
                p.completion_time,
                p.turnaround_time,
                p.waiting_time,
            );
        }

        println!("|---------|----|----|----------|----|-----|-----|");
        println!("Average Turnaround Time: {:.2}", self.avg_tat);
        println!("Average Waiting Time: {:.2}", self.avg_wt);

        self.display_gantt_chart();
    }
}