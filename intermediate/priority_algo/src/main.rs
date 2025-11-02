/*
 * Priority Algorithm
 * A method of scheduling processes that is based on priority.
 * In this algorithm, the scheduler selects the tasks to work as per the priority.
 *
 * The processes with higher priority should be carried out first, whereas jobs
 * with equal priorities are carried out on a round-robin or FCFS basis.
 * Priority depends upon memory requirements, time requirements, and other factors.
 *
 * Types of Priority Scheduling:
 * 1. Preemptive Scheduling
 * The tasks are mostly assigned with their priorities. Sometimes it is important to run
 * a task with a higher priority before another lower priority task, even if the lower
 * priority task is still running. The lower priority task holds for some time and resumes
 * when the higher priority task finished its execution.
 *
 * 2. Non-Preemptive Scheduling
 * The CPU has been allocated to a specific process. The process that keeps the CPU busy,
 * will release the CPU either by switching context or terminating. It is the only method
 * that can be used for various hardware platforms. That's because it doesn't need special
 * hardware (for example, a timer) like preemptive scheduling.
 *
 * Characteristics of Priority Scheduling:
 * 1. A CPU algorithm that schedules processes based on priority.
 * 2. It is used in Operating Systems for performing batch processes.
 * 3. If two jobs having the same priority are READY, it works on a FCFS basis.
 * 4. In priority scheduling, a number is assigned to each process that indicates its priority.
 * 5. The lower the number, the higher the priority.
 * 6. In this type of scheduling, if a newer process arrives with a higher priority than the currently
 *   running process, the CPU is preempted and assigned to the newer process.
 *
 */

mod models;
mod scheduler;
mod utils;

use crate::{
    models::priority_process::Process,
    scheduler::priority_scheduler::{Scheduler, PriorityScheduler},
    utils::input::{user_input, valid_input},
    utils::try_again::try_again
};

fn main() {
    println!("Hello, world!");
}
