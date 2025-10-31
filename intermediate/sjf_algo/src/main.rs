/*
 * Shortest Job First (SJF) Algorithm
 * The processes are scheduled in ascending order of their CPU burst times,
 * i.e, the CPU is allocated to the process with the shortest execution time.
 *
 * SJF (non-preemptive) scheduling
 * Once a process is assigned to the CPU, it runs to completion.
 * Here, the short term scheduler is invoked when a process
 * completes its execution or when a new process/es arrives in
 * an empty ready queue.
 *
 * SJF (preemptive) scheduling
 * Also known as Shortest Remaining Time First (SRTF).
 * Here, if a short prcess enters the ready queue while a long
 * process is executing, process switch the newly arrived shorter
 * process starts to execute.
 *
 * Features of SJF Algorithm
 * 1. Allocates CPU to the process with shortest execution time.
 * 2. In cases where two or more processes have the same burst time,
 * arbitration is done among these processes on first come, first serve basis.
 * 3. There are both preemptive and non-preemptive versions.
 * 4. It minimizes the average waiting time of the processes.
 * 5. It may cause starvation of long processes if short processes
 * continue to come in the system.
 */
mod models;
mod scheduler;
mod utils;

use crate::models::process::Process;
use crate::scheduler::scheduler::{SJFPreemptiveScheduler, Scheduler};
use crate::utils::input::{user_input, valid_input};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("SJF Preemptive Scheduling Algorithm");

        let num_input = user_input("Enter number of process (3-5): ")?;
        let num_of_processes: usize = match num_input.trim().parse() {
            Ok(num) if (3..=5).contains(&num) => num,
            _ => {
                println!("Please enter a number between 3 and 5.");
                continue;
            }
        };

        let mut processes = Vec::new();
        for i in 0..num_of_processes {
            println!("Process {}", i + 1);

            let arrival_time = loop {
                let input = user_input("Enter Arrival Time: ")?;
                match valid_input(&input) {
                    Ok(v) => break v,
                    Err(e) => println!("{}", e),
                }
            };

            let burst_time = loop {
                let input = user_input("Enter Burst Time: ")?;
                match valid_input(&input) {
                    Ok(v) => break v,
                    Err(e) => println!("{}", e),
                }
            };

            processes.push(Process::new(i + 1, arrival_time, burst_time));
        }

        let mut scheduler = SJFPreemptiveScheduler::new(processes);
        scheduler.schedule();
        scheduler.display();

        loop {
            let try_again = user_input("\nTry again (yes or no)?: ")?;
            match try_again.trim().to_lowercase().as_str() {
                "yes" => break,
                "no" => {
                    println!("SJF Algorithm Simulation done, Bye!");
                    return Ok(());
                }
                _ => {
                    print!("Please choose between 'yes' or 'no': ");
                    continue;
                }
            }
        }
    }
}
