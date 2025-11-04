use crate::{
    models::cpu_process::SJFProcess,
    utils::gantt_display::{GanttChart, GanttSegment}
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

        while completed_process < num_of_processes {
            let mut idx: Option<usize> = None;
            let mut min_rt = u32::MAX;

            for (i, p) in self.processes.iter().enumerate() {
                if p.base.arrival_time <= time && p.remaining_time > 0 && p.remaining_time < min_rt {
                    min_rt = p.remaining_time;
                    idx = Some(i);
                }
            }

            match idx {
                Some(i) => {
                    let process = &mut self.processes[i];
                    process.remaining_time -= 1;

                    if process.remaining_time == 0 {
                        process.completion_time = time + 1;
                        process.turnaround_time = process.completion_time - process.base.arrival_time;
                        process.waiting_time = process.turnaround_time - process.base.burst_time;
                        completed_process += 1;
                    }

                    self.gantt_chart.segments.push(GanttSegment {
                        pid: process.base.pid,
                        start_time: time,
                        end_time: time + 1,
                    });

                }
                None => {
                    if let Some(next_arrival) = self.processes
                        .iter()
                        .filter(|p| p.remaining_time > 0)
                        .map(|p| p.base.arrival_time)
                        .min()
                    {
                        time = next_arrival;
                    } else {
                        break;
                    }
                }
            }

            time += 1;
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