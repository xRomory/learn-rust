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

fn main() {
    println!("Hello, world!");
}
