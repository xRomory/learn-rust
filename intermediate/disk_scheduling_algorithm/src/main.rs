use crate::{
    algorithms::fcfs::{BasicFCFS, ClusteredFCFS}, request_patterns::{ClusteredPattern, RandomPattern, SequentialPattern}, simulator::DiskSimulator, types::DiskConfig
};

mod algorithms;
mod request_patterns;
mod simulator;
mod types;

fn main() {
    let config = DiskConfig {
        initial_position: 50,
        total_tracks: 200,
        max_track: 199
    };

    let simulator = DiskSimulator::new(config);

    let clustered_pattern = ClusteredPattern::new(vec![
        (30, 10),
        (80, 5),
        (150, 15),
    ]);

    let random_pattern = RandomPattern;
    let sequential_pattern = SequentialPattern::new(0, 10);

    let basic_fcfs = BasicFCFS;
    let clusterd_fcfs = ClusteredFCFS::new(15);

    println!("Disk Scheduling Algorithms Simulation\n");

    // Different Scenarios for Basic FCFS
    simulator.run_simulation(&clustered_pattern, &basic_fcfs, 12);
    simulator.run_simulation(&random_pattern, &basic_fcfs, 8);
    simulator.run_simulation(&sequential_pattern, &basic_fcfs, 10);

    // Different Scenarios for Clustered FCFS
    simulator.run_simulation(&clustered_pattern, &clusterd_fcfs, 12);
    simulator.run_simulation(&random_pattern, &clusterd_fcfs, 8);
    simulator.run_simulation(&sequential_pattern, &clusterd_fcfs, 10);
}
