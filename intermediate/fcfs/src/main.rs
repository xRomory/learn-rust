/*
 * FCFS (First-Come, First-Served) Scheduling Algorithm Implementation in Rust
 *
 * FCFS is a non-preemptive CPU scheduling algorithm
 * where the process that arrives first in the ready queue
 * is executed first by the CPU.
 *
 * Characteristics:
 * Non-preemptive: Once a process starts, it runs to completion.
 * Based on arrival time: The process with the earliest arrival time gets the CPU first.
 * Simple and fair, but can lead to long waiiting times for shorter processes.
 *
 * This is the first activity during my OS course.
 * * The goal for this project is to convert Java implementation to Rust
 * * Learn more and dig deeper to Rust
 */

use std::io::{self, Write};

// Define a `struct` to represent a process
#[derive(Debug, Clone)]
struct Process {
    pid: usize,
    arrival_time: i32,
    burst_time: i32,
    start_time: i32,
    completion_time: i32,
    turnaround_time: i32,
    waiting_time: i32,
}

impl Process {
    fn new(pid: usize, arrival_time: i32, burst_time: i32) -> Self {
        Process {
            pid,
            arrival_time,
            burst_time,
            start_time: 0,
            completion_time: 0,
            turnaround_time: 0,
            waiting_time: 0,
        }
    }
}

trait Scheduler {
    fn schedule(&mut self);
    fn display(&self);
}

struct FCFSScheduler {
    processes: Vec<Process>,
    avg_tat: f32,
    avg_wt: f32,
}

impl FCFSScheduler {
    fn new(processes: Vec<Process>) -> Self {
        Self {
            processes,
            avg_tat: 0.0,
            avg_wt: 0.0,
        }
    }
}

impl Scheduler for FCFSScheduler {
    fn schedule(&mut self) {
        // Sort by arrival time
        self.processes.sort_by_key(|p| p.arrival_time);
        let mut current_time = 0;
        let mut total_tat = 0;
        let mut total_wt = 0;

        for p in &mut self.processes {
            let at = p.arrival_time;
            let bt = p.burst_time;

            p.start_time = if current_time < at { at } else { current_time };
            p.completion_time = p.start_time + bt;
            p.turnaround_time = p.completion_time - at;
            p.waiting_time = p.turnaround_time - bt;

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
                p.pid,
                p.arrival_time,
                p.burst_time,
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
            print!(" P{} |", p.pid);
        }
        println!();

        print!("0");
        for p in &self.processes {
            print!("    {}", p.completion_time);
        }
        println!();
    }
}

fn valid_at_bt(input: &str) -> Result<i32, &'static str> {
    match input.trim().parse::<i32>() {
        Ok(num) if num >= 0 => Ok(num),
        Ok(_) => Err("Please enter a non-negative number"),
        Err(_) => Err("Invalid input. Please enter a valid number."),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // For example (Given in PDF file)
    // Uncomment to test with predefined processes
    // let processes = vec![
    //     Process::new(1, 0, 4),
    //     Process::new(2, 1, 3),
    //     Process::new(3, 2, 1),
    //     Process::new(4, 3, 2),
    //     Process::new(5, 5, 4),
    // ];
    // let mut scheduler = FCFSScheduler::new(processes);
    // scheduler.schedule();
    // scheduler.display();

    loop {
        print!("Enter the number of process: ");
        io::stdout().flush()?;
        let mut num_proc_input = String::new();
        io::stdin().read_line(&mut num_proc_input)?;
        match num_proc_input.trim().parse::<usize>() {
            Ok(num) if num > 0 => {
                let mut processes = Vec::new();
                let mut i = 0;
                while i < num {
                    println!("Process: {}", i + 1);
                    print!("Enter Arrival Time: ");
                    io::stdout().flush()?;
                    let mut at_input = String::new();
                    io::stdin().read_line(&mut at_input)?;

                    let arrival_time = match valid_at_bt(&at_input) {
                        Ok(at) => at,
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        }
                    };

                    print!("Enter Burst Time: ");
                    io::stdout().flush()?;
                    let mut bt_input = String::new();
                    io::stdin().read_line(&mut bt_input)?;
                    let burst_time = match valid_at_bt(&bt_input) {
                        Ok(bt) => bt,
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        }
                    };
                    println!();

                    processes.push(Process::new(i + 1, arrival_time, burst_time));
                    i += 1;
                }

                let mut scheduler = FCFSScheduler::new(processes);
                scheduler.schedule();
                scheduler.display();

                print!("\nDo you want to try again (yes or no)?: ");
                io::stdout().flush()?;
                loop {
                    let mut repeat_input = String::new();
                    io::stdin().read_line(&mut repeat_input)?;
                    match repeat_input.trim().to_lowercase().as_str() {
                        "yes" => break,
                        "no" => {
                            println!("FCFC Algorithm Simulation done. Bye!");
                            return Ok(());
                        }
                        _ => {
                            print!("Please choose between 'yes' or 'no' only: ");
                            io::stdout().flush()?;
                            continue;
                        }
                    }
                }
                // break;
            }
            Ok(_) => {
                println!("Please enter a number greater than 0");
                continue;
            }
            Err(_) => {
                println!("Invalid input! Please enter a valid integer.");
                continue;
            }
        }
    }
}
