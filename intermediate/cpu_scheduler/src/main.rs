mod models;
mod scheduler;
mod utils;

use crate:: {
    models::cpu_process::{BaseProcess, FCFSProcess, SJFProcess},
    scheduler::{fcfs::{FCFSScheduler, Scheduler}, sjf::{SJFPreemptiveScheduler, SJFScheduler}},
    utils::{input::{get_processes_from_user, user_input}, try_again::try_again}
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
            6 => {
                println!("CPU Scheduling Algorithm Exiting... Bye!");
                break
            },
            _ => println!("\nInvalid option, try again.")
        }
    }

    Ok(())
}
