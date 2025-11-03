/*
 * Highest Response Ratio Next (HRRN) Scheduling Algorithm
 * A non-preemptive in which, HRRN scheduling is done based
 * on an extra parameter called Response Ratio (RR).
 * Given N processes with their arrival time and burst time,
 * the task is to find the average waiting time and
 * an average turnaround time using HRRN scheduling algorithm.
 *
 * The name itself states that we need to find the response ratio
 * of all available processes and select the one with the highest
 * response ratio. It is designed to improve upon simpler algorithms
 * like First-Come, First-Served (FCFS) and Shortest Job Next (SJN)
 * by balancing both the waiting time and the burst time of processes.
 * A process once selected will run till completion.
 *
 * Response Ratio (RR) is calculated using the formula:
 * RR = (Waiting Time + Burst Time per service time) / Burst Time
 *
 * Characteristics of HRRN Scheduling Algorithm:
 * 1. a non-preemptive CPU scheduling algorithm and it is considered
 *   as one of the most optimal scheduling algorithms.
 * 2. The criteria for HRRN is Response Ratio (RR), and the mode is
 *   non-preemptive.
 * 3. HRRN is considered as the modification of the SJF scheduling
 *  to reduce the starvation problem.
 * 4. In comparison with SJF, during the HRRN scheduling, the CPU is
 *  allocated to the process with the highest response ratio, and
 *  not to the process having less burst time.
 */

mod models;
mod scheduler;
mod utils;

use crate::{
    models::hrrn_process::Process,
    scheduler::hrrn::{HRRNScheduler, Scheduler},
    utils::input::{user_input, valid_input},
    utils::try_again::try_again,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("HRRN Scheduling Algorithm\n");

        let num_input = user_input("Enter the number of processes (3-5): ")?;
        let num_of_processes: usize = match num_input.trim().parse() {
            Ok(num) if (3..=5).contains(&num) => num,
            _ => {
                println!("Please enter between 3 and 5 only");
                continue;
            }
        };

        let mut processes = Vec::new();
        for i in 0..num_of_processes {
            println!("\nProcess {}", i + 1);

            let arrival_time = loop {
                let input = user_input("Enter Arrival Time: ")?;
                match valid_input(&input) {
                    Ok(value) => break value,
                    Err(e) => println!("{}", e),
                }
            };

            let burst_time = loop {
                let input = user_input("Enter Burst Time: ")?;
                match valid_input(&input) {
                    Ok(value) => break value,
                    Err(e) => println!("{}", e),
                }
            };

            processes.push(Process::new(i + 1, arrival_time, burst_time));
        }

        let mut scheduler = HRRNScheduler::new(processes);
        scheduler.scheduler();
        scheduler.display();

        let again = try_again()?;
        if !again {
            break;
        }
    }

    Ok(())
}
