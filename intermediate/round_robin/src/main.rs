/*
 * Round Robin Scheduling Algorithm
 * Commonly used in time-sharing systems. The algorithm works
 * by a fixed time slice or quantum to each process in the ready queue.
 * When a process's time quantum expires, the CPU is preempted and
 * the next process in the queue gets a turn.
 *
 * How it works:
 * 1. Process Queue:
 * All process are placed in a queue. When the CPU becomes idle,
 * the first process in the queue is selected to use the CPU.
 *
 * 2. Time Quantum (or Time Slice):
 * Each process is assigned a fixed time slice or quantum (e.g., 10ms).
 * The proccess will run for the duration of the time quantum, or
 * until it completes its execution, whichever comes first.
 *
 * 3. Context Switching:
 * When the time quantum expires, the CPU is switched to the next process
 * in the queue, regardless of whether the current process is finished.
 * The current process is then put back at the end of the queue.
 *
 * This is called a context switch, and the process continues its execution
 * in the next round if it hasn't completed.
 *
 * 4. Fairness and Responsiveness:
 * All processes receive an equal opportunity to execute, which means
 * Round Robin is considered fair because each process gets a turn
 * to use the CPU. This makes it suitable for time-sharing systems
 * where responsiveness is crucial.
 *
 */

mod models;
mod scheduler;
mod utils;

use crate::{
    models::rr_process::Process, scheduler::round_robin::{RoundRobinScheduler, Scheduler}, utils::{input::{user_input, valid_input}, try_again::try_again}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Round Robin Scheduling Algorithm");
        let num_input = user_input("Enter number of processes (3-5): ")?;

        let num_of_processes: usize = match num_input.trim().parse() {
            Ok(num) if (3..=5).contains(&num) => num,
            _ => {
                println!("Please enter between 3 and 5.");
                continue;
            }
        };

        let mut processes = Vec::new();
        for i in 0..num_of_processes {
            println!("Process {}", i + 1);

            let arrival_time: u32 = loop {
                let input = user_input("Enter Arrival Time: ")?;
                match valid_input(&input) {
                    Ok(v) => break v as u32,
                    Err(e) => println!("{}", e)
                }
            };

            let burst_time: u32 = loop {
                let input = user_input("Enter Burst Time: ")?;
                match valid_input(&input) {
                    Ok(v) => break v as u32,
                    Err(e) => println!("{}", e)
                }
            };

            processes.push(Process::new(i + 1, arrival_time, burst_time));
        }

        let time_quantum: u32 = loop {
            let input = user_input("\nEnter Quantum Time: ")?;
            match valid_input(&input) {
                Ok(v) => break v as u32,
                Err(e) => println!("{}", e)
            }
        };

        let mut scheduler = RoundRobinScheduler::new(time_quantum);
        for p in processes {
            scheduler.add_process(p);
        }
        scheduler.schedule();
        scheduler.display();

        let again = try_again()?;

        if !again {
            break;
        }
    }

    Ok(())
}
