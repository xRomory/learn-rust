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

fn main() {
    println!("Hello, world!");
}
