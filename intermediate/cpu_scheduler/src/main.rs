mod models;
mod scheduler;
mod utils;

use crate:: {
    models::cpu_process::{BaseProcess, FCFSProcess, RRProcess, SJFProcess},
    scheduler::{fcfs::{FCFSScheduler, Scheduler}, priority::{PrioritySched, PriorityScheduler}, round_robin::{RRScheduler, RoundRobinScheduler}, sjf::{SJFPreemptiveScheduler, SJFScheduler}},
    utils::{
        input::{get_priority_processes_from_user, get_processes_from_user, user_input, valid_input},
        try_again::try_again
    }
};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    loop {
        println!("CPU Scheduling Algorithms Simulation");
        println!("1. FCFS Algorithm");
        println!("2. Shortest Job First Algorithm");
        println!("3. Round Robin Algorithm");
        println!("4. Priority Scheduling Algorithm");
        println!("5. Highest Response Ratio Next Algorithm");
        println!("6. Exit");
        let option = user_input("Enter your choice (1-6): ")?;
        let valid_option: u8 = match option.trim().parse::<u8>() {
            Ok(num) if (1..=6).contains(&num) => num,
            _ => {
                println!("Please choose between 1 to 6 only.");
                continue;
            }
        };

        match valid_option {
            1 => {
                'fcfc_loop: loop {
                    println!("\nFCFS Algorithm Simulation\n");
                    let base_process: Vec<BaseProcess> = get_processes_from_user()?;
                    let fcfs_processes: Vec<FCFSProcess> = base_process.into_iter().map(FCFSProcess::new).collect();

                    let mut fcfs_scheduler = FCFSScheduler::new(fcfs_processes);
                    fcfs_scheduler.schedule();
                    fcfs_scheduler.display();

                    let again: bool = try_again()?;
                    if again {
                        continue 'fcfc_loop
                    } else {
                        break 'fcfc_loop
                    }
                }
            },
            2 => {
                'sjf_loop: loop {
                    println!("\nShortest Job First (Preemptive) Algorithm Simulation\n");
                    let base_process: Vec<BaseProcess> = get_processes_from_user()?;
                    let sjf_processes: Vec<SJFProcess> = base_process.into_iter().map(SJFProcess::new).collect();

                    let mut sjf_scheduler = SJFPreemptiveScheduler::new(sjf_processes);
                    sjf_scheduler.schedule();
                    sjf_scheduler.display();

                    let again: bool = try_again()?;
                    if again {
                        continue 'sjf_loop
                    } else {
                        break 'sjf_loop
                    }
                }
            },
            3 => {
                'rr_loop: loop {
                    println!("\nRound Robin Algorithm Simulation\n");
                    let base_process: Vec<BaseProcess> = get_processes_from_user()?;
                    let time_quantum: u32 = loop {
                        let input = user_input("\nEnter Quantum Time: ")?;
                        match valid_input(&input) {
                            Ok(v) => break v as u32,
                            Err(e) => println!("{}", e)
                        }
                    };

                    let rr_processes: Vec<RRProcess> = base_process.into_iter().map(RRProcess::new).collect();

                    let mut rr_scheduler = RoundRobinScheduler::new(time_quantum, rr_processes);
                    rr_scheduler.schedule();
                    rr_scheduler.display();

                    let again: bool = try_again()?;
                    if again {
                        continue 'rr_loop
                    } else {
                        break 'rr_loop
                    }
                }
            },
            4 => {
                'priority_loop: loop {
                    println!("\nNon-Preemptive Priority CPU Scheduling Algorithm Simulation\n");
                    let priority_process = get_priority_processes_from_user()?;
                    
                    let mut priority_scheduler  = PriorityScheduler::new(priority_process);
                    priority_scheduler.schedule();
                    priority_scheduler.display();

                    let again: bool = try_again()?;
                    if again {
                        continue 'priority_loop
                    } else {
                        break 'priority_loop
                    }
                }
            }
            6 => {
                println!("CPU Scheduling Algorithm Exiting... Bye!");
                break
            },
            _ => println!("\nInvalid option, try again.")
        }
    }

    Ok(())
}
