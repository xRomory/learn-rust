use deadlock_detection::DeadlockSimulator;

fn main() {
    println!("Deadlock Detection and Prevention System");
    println!("========================================");

    let available = vec![3, 3, 2];

    let max = vec![
        vec![7, 5, 3],
        vec![3, 2, 2],
        vec![9, 0, 2],
        vec![2, 2, 2],
        vec![4, 3, 3],
    ];

    let allocation = vec![
        vec![0, 1, 0],
        vec![2, 0, 0],
        vec![3, 0, 2],
        vec![2, 1, 1],
        vec![0, 0, 2],
    ];

    let simulator = DeadlockSimulator::new(available, max, allocation);

    simulator.run_banker_simulation();
    simulator.run_avoidance_simulation();

    println!("\n=== Simulation Complete ===");
}
