use crate::models::process::Process;

pub trait Scheduler {
    fn schedule(&mut self);
    fn display(&self);
}

pub struct SJFPreemptiveScheduler {
    pub processes: Vec<Process>,
    pub avg_tat: f32,
    pub avg_wt: f32,
}

impl SJFPreemptiveScheduler {
    pub fn new(processes: Vec<Process>) -> Self {
        Self {
            processes,
            avg_tat: 0.0,
            avg_wt: 0.0,
        }
    }
}

impl Scheduler for SJFPreemptiveScheduler {
    fn schedule(&mut self) {
        let mut time = 0;
        let mut completed = 0;
        let num_of_process = self.processes.len();
        let mut gantt_chart = Vec::new();

        let mut procs = self.processes.clone();

        while completed < num_of_process {
            let mut idx: Option<usize> = None;
            let mut min_rt = i32::MAX;

            for (i, p) in procs.iter().enumerate() {
                if p.arrival_time <= time && p.remaining_time > 0 && p.remaining_time < min_rt {
                    min_rt = p.remaining_time;
                    idx = Some(i);
                }
            }

            if let Some(i) = idx {
                procs[i].remaining_time -= 1;
                gantt_chart.push(procs[i].pid);

                if procs[i].remaining_time == 0 {
                    procs[i].completion_time = time + 1;
                    procs[i].turnaround_time = procs[i].completion_time - procs[i].arrival_time;
                    procs[i].waiting_time = procs[i].turnaround_time - procs[i].burst_time;
                    completed += 1;
                }
            } else {
                if let Some(next_arrival) = procs
                    .iter()
                    .filter(|p| p.remaining_time > 0)
                    .map(|p| p.arrival_time)
                    .min()
                {
                    time = next_arrival;
                } else {
                    break;
                }
            }

            time += 1;
        }

        self.avg_tat =
            procs.iter().map(|p| p.turnaround_time).sum::<i32>() as f32 / num_of_process as f32;
        self.avg_wt =
            procs.iter().map(|p| p.waiting_time).sum::<i32>() as f32 / num_of_process as f32;
        self.processes = procs;

        // Store Gantt Chart
        println!("Gantt Chart:");
        print!("|");
        for pid in &gantt_chart {
            print!(" P{} |", pid);
        }
        println!();

        print!("0");
        for i in 0..gantt_chart.len() {
            if i == gantt_chart.len() - 1 || gantt_chart[i] != gantt_chart[i + 1] {
                print!("   {}", i + 1);
            }
        }
        println!("\n");
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
                p.pid,
                p.arrival_time,
                p.burst_time,
                p.completion_time,
                p.turnaround_time,
                p.waiting_time,
                "",
            )
        }

        println!("|---------|----|----|----|----|-----|----|");
        println!("\nAverage Turnaround Time: {:.2}", self.avg_tat);
        println!("\nAverage Waiting Time: {:.2}", self.avg_wt);
    }
}
