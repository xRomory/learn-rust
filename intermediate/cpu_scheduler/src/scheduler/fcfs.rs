use crate::models::cpu_process::FCFSProcess;

pub trait Scheduler {
    fn schedule(&mut self);
    fn display(&self);
}

pub struct FCFSScheduler {
    pub processes: Vec<FCFSProcess>,
    pub avg_tat: f32,
    pub avg_wt: f32,
}

impl FCFSScheduler {
    pub fn new(processes: Vec<FCFSProcess>) -> Self {
        Self {
            processes,
            avg_tat: 0.0,
            avg_wt: 0.0,
        }
    }
}

impl Scheduler for FCFSScheduler {
    fn schedule(&mut self) {
        self.processes.sort_by_key(|p: &FCFSProcess| p.base.arrival_time);

        let mut current_time = 0;
        let mut total_tat = 0;
        let mut total_wt = 0;

        for p in &mut self.processes {
            p.start_time = if current_time < p.base.arrival_time { p.base.arrival_time } else { current_time };
            p.completion_time = p.start_time + p.base.burst_time;
            p.turnaround_time = p.completion_time - p.base.arrival_time;
            p.waiting_time = p.turnaround_time - p.base.burst_time;

            current_time = p.completion_time;

            total_tat += p.turnaround_time;
            total_wt += p.waiting_time;
        }

        self.avg_tat = total_tat as f32 / self.processes.len() as f32;
        self.avg_wt = total_wt as f32 / self.processes.len() as f32;
    }

    fn display(&self) {
        println!("|---------|----|----|----|----|-----|----|");
        println!(
            "|{:^9}|{:^4}|{:^4}|{:^4}|{:^4}|{:^5}|{:^4}|",
            "Process", "AT", "BT", "ST", "CT", "TAT", "WT"
        );
        println!("|---------|----|----|----|----|-----|----|");

        for p in &self.processes {
            println!(
                "|{:^9}|{:^4}|{:^4}|{:^4}|{:^4}|{:^5}|{:^4}|",
                p.base.pid,
                p.base.arrival_time,
                p.base.burst_time,
                p.start_time,
                p.completion_time,
                p.turnaround_time,
                p.waiting_time,
            );
        }

        println!("|---------|----|----|----|----|-----|----|");

        println!("\nAverage TAT: {:.2}", self.avg_tat);
        println!("Average WT: {:.2}", self.avg_wt);

        // Gantt Chart
        print!("\nGantt Chart:\n|");
        for p in &self.processes {
            print!(" P{} |", p.base.pid);
        }
        println!();

        print!("0");
        for p in &self.processes {
            print!("    {}", p.completion_time);
        }
        println!();
    }
}
