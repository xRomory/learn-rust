use crate::{
    algorithms::{
        cscan::{CSCAN, CScanDirection}, fcfs::{BasicFCFS, ClusteredFCFS}, scan::{SCAN, ScanDirection}, sstf::{ClusteredSSTF, SSTF}
    }, 
    request_patterns::{ClusteredPattern, RandomPattern, SequentialPattern}, 
    simulator::DiskSimulator, types::DiskConfig
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

    let sstf = SSTF;
    let clusterd_sstf = ClusteredSSTF::new(15);

    let scan_right = SCAN::new(ScanDirection::RIGHT);
    let scan_left = SCAN::new(ScanDirection::LEFT);

    let cscan_right = CSCAN::new(CScanDirection::RIGHT);
    let cscan_left = CSCAN::new(CScanDirection::LEFT);

    println!("Disk Scheduling Algorithms Simulation\n");

    // Simulation for FCFS
    simulator.run_simulation(&clustered_pattern, &basic_fcfs, 12);
    simulator.run_simulation(&clustered_pattern, &clusterd_fcfs, 12);

    simulator.run_simulation(&random_pattern, &basic_fcfs, 8);
    simulator.run_simulation(&random_pattern, &clusterd_fcfs, 8);
    
    simulator.run_simulation(&sequential_pattern, &basic_fcfs, 10);
    simulator.run_simulation(&sequential_pattern, &clusterd_fcfs, 10);

    // Simulation for SSTF
    simulator.run_simulation(&clustered_pattern, &sstf, 12);
    simulator.run_simulation(&clustered_pattern, &clusterd_sstf, 12);

    simulator.run_simulation(&random_pattern, &sstf, 8);
    simulator.run_simulation(&random_pattern, &clusterd_sstf, 8);
    
    simulator.run_simulation(&sequential_pattern, &sstf, 10);
    simulator.run_simulation(&sequential_pattern, &clusterd_sstf, 10);

    // Simulation for SCAN
    simulator.run_simulation(&clustered_pattern, &scan_right, 12);
    simulator.run_simulation(&clustered_pattern, &scan_left, 12);

    simulator.run_simulation(&random_pattern, &scan_right, 8);
    simulator.run_simulation(&random_pattern, &scan_left, 8);

    simulator.run_simulation(&sequential_pattern, &scan_right, 10);
    simulator.run_simulation(&sequential_pattern, &scan_left, 10);

    // Simulation for CSCAN
    simulator.run_simulation(&clustered_pattern, &cscan_right, 12);
    simulator.run_simulation(&clustered_pattern, &cscan_left, 12);

    simulator.run_simulation(&random_pattern, &cscan_right, 8);
    simulator.run_simulation(&random_pattern, &cscan_left, 8);

    simulator.run_simulation(&sequential_pattern, &cscan_right, 10);
    simulator.run_simulation(&sequential_pattern, &cscan_left, 10);
}
