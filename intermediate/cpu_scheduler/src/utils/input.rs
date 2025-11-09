use std::io::{self, Write};

use crate::models::cpu_process::{BaseProcess, PriorityProcess};

pub fn user_input(message: &str) -> Result<String, io::Error> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn valid_input(input: &str) -> Result<u32, &'static str> {
    input.trim().parse::<u32>().map_err(|_| "Invalid input. Please enter a positive integer.")
}

pub fn get_processes_from_user() -> Result<Vec<BaseProcess>, Box<dyn std::error::Error>> {
    let num_of_processes = loop {
        let proc_input = user_input("Enter the number of processes (3-5): ")?;
        match proc_input.trim().parse() {
            Ok(num) if (3..=5).contains(&num) => break num,
            _ => println!("Please choose between 3 to 5 only.")
        };
    };

    let mut processes = Vec::new();
    for i in 0..num_of_processes {
        println!("Process {}", i + 1);

        let arrival_time = loop {
            let input = user_input("Enter Arrival Time: ")?;
            match valid_input(&input) {
                Ok(value) => break value,
                Err(e) => println!("{}", e)
            }
        };
        let burst_time = loop {
            let input = user_input("Enter Burst Time: ")?;
            match valid_input(&input) {
                Ok(value) => break value,
                Err(e) => println!("{}", e)
            }
        };

        processes.push(BaseProcess::new(i + 1, arrival_time, burst_time));
    }
    
    Ok(processes)
}

pub fn get_priority_processes_from_user() -> Result<Vec<PriorityProcess>, Box<dyn std::error::Error>> {
    let num_of_processes = loop {
        let proc_input = user_input("Enter the number of processes (3-5): ")?;
        match proc_input.trim().parse() {
            Ok(num) if (3..=5).contains(&num) => break num,
            _ => println!("Please choose between 3 to 5 only.")
        };
    };

    let mut processes = Vec::new();
    for i in 0..num_of_processes {
        println!("Process {}", i + 1);

        let arrival_time = loop {
            let input = user_input("Enter Arrival Time: ")?;
            match valid_input(&input) {
                Ok(value) => break value,
                Err(e) => println!("{}", e)
            }
        };
        let burst_time = loop {
            let input = user_input("Enter Burst Time: ")?;
            match valid_input(&input) {
                Ok(value) => break value,
                Err(e) => println!("{}", e)
            }
        };

        let priority: u32 = loop {
            let input = user_input("Enter Priority (lower = higher): ")?;
            match valid_input(&input) {
                Ok(value) => break value,
                Err(e) => println!("{}", e)
            }
        };

        let bp = BaseProcess::new(i + 1, arrival_time, burst_time);

        processes.push(PriorityProcess::new(bp, priority));
    }
    
    Ok(processes)
}