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
    models::process::Process,
    utils::input::{user_input, valid_input},
    utils::try_again::try_again,
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

        try_again()?;
    }
}
