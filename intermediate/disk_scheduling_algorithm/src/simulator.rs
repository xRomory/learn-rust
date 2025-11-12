use crate::types::{DiskScheduler, RequestPattern, DiskConfig, SimulationResult, DiskRequest};

#[derive(Debug)]
pub struct DiskSimulator {
    config: DiskConfig
}

impl DiskSimulator {
    pub fn new(config: DiskConfig) -> Self {
        Self { config }
    }

    pub fn run_simulation<P, S>(
        &self,
        pattern: &P,
        scheduler: &S,
        request_count: usize,
    ) -> SimulationResult
    where
        P: RequestPattern,
        S: DiskScheduler
    {
        println!("=== Running Simulation ===");
        println!("Pattern: {}", pattern.name());
        println!("Algorithm: {}", scheduler.name());
        println!("Request Count: {}", request_count);
        println!("Initial Position: {}", self.config.initial_position);
        println!("Total Tracks: {}", self.config.total_tracks);
        println!();

        let requests = pattern.generate_requests(request_count, &self.config);
        let result = scheduler.schedule(&requests, &self.config);

        self.print_simulation_details(&requests, &result);
        result

    }

    fn print_simulation_details(
        &self,
        _requests: &[DiskRequest],
        result: &SimulationResult
    ) {
        println!("=== Simulation Results ===");
        println!("Algorithm (from result): {}", result.algorithm_name);
        println!("Total Seek Time: {}", result.total_seek_time);
        println!("Average Seek Time: {:.2}", result.average_seek_time);
        println!("Requests Processed: {}", result.requests_processed);
        println!("Seek Sequence: {:?}", result.seek_sequence);

        let throughput = if result.total_seek_time > 0 {
            (result.requests_processed as f64 / result.total_seek_time as f64) * 100.0
        } else {
            0.0
        };

        println!("Throughput: {:.2} request per 100 seeks", throughput);
        println!();
    }
}