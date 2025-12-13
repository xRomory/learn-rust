use rand::{Rng, rngs::ThreadRng};

use crate::{
    detection::{BankerAlgorithm, DeadlockDetector}, models::SystemState, prevention::{DeadlockAvoidance, DeadlockPreventor, PreemptiveAllocator}, rag::ResourceAllocationGraph
};

#[derive(Debug)]
pub struct DeadlockSimulator {
    state: SystemState,
    max_requests: usize,
}

impl DeadlockSimulator {
    pub fn new(
        available: Vec<usize>,
        max: Vec<Vec<usize>>,
        allocation: Vec<Vec<usize>>
    ) -> Self {
        let state = SystemState::new(available, max, allocation);

        Self {
            state,
            max_requests: 10
        }
    }

    pub fn run_banker_simulation(&self) {
        println!("\n=== Banker's Algorithm Simulation ===");

        let mut banker = BankerAlgorithm::new(self.state.clone());

        println!("Initial State:");
        self.print_state(&banker.state);

        println!("Initial safe sequence: {:?}", banker.get_safe_sequence());

        // Simulate some requests
        let mut rng: ThreadRng = rand::rng();

        for _ in 0..self.max_requests {
            let process_id = rng.random_range(0..self.state.processes);
            let request: Vec<usize> = (0..self.state.resources)
                .map(|_| rng.random_range(0..2))
                .collect();

            println!("\nProcess {} requests: {:?}", process_id, request);

            match banker.request_resource(process_id, request.clone()) {
                Ok(()) => {
                    println!("Request granted");
                    println!("New safe sequence: {:?}", banker.get_safe_sequence());
                }
                Err(e) => {
                    println!("Request denied: {}", e);
                }
            }
        }

        // Check for deadlock
        if let Some(deadlocked) = banker.detect_deadlock() {
            println!("\nDeadlock detected! Deadlocked processes: {:?}", deadlocked);
        } else {
            println!("\nNo deadlock detected");
        }
    }

    pub fn run_avoidance_simulation(&self) {
        println!("\n=== Deadlock Avoidance Simulation ===");

        let mut avoidance = DeadlockAvoidance::new(self.state.clone());

        println!("Initial State:");
        self.print_state(&avoidance.get_state());

        let mut rng: ThreadRng = rand::rng();

        for i in 0..self.max_requests {
            let process_id = rng.random_range(0..self.state.processes);
            let request: Vec<usize> = (0..self.state.resources)
                .map(|_| rng.random_range(0..2))
                .collect();

            println!("\nRequest {}: Process {} requests{:?}", i, process_id, request);

            if avoidance.can_allocate(process_id, &request) {
                match avoidance.allocate(process_id, request) {
                    Ok(()) => println!("Allocation successful"),
                    Err(e) => println!("Allocation failed: {}", e),
                }
            } else {
                println!("Allocation would lead to unsafe state - request denied");
            }
        }
    }

    pub fn run_preemption_simulation(&self) {
        println!("\n=== Preemptive Allocation Simulation ===");

        let mut preempt = PreemptiveAllocator::new(self.state.clone());

        println!("Initial State:");
        self.print_state(&preempt.state);

        let mut rng: ThreadRng = rand::rng();

        for i in 0..self.max_requests {
            let process_id = rng.random_range(0..self.state.processes);
            let request: Vec<usize> = (0..self.state.resources)
                .map(|j| {
                    let max_request = preempt.state.need[process_id][j];
                    if max_request > 0 {
                        rng.random_range(0..=max_request.min(3))
                    } else {
                        0
                    }
                })    // Larger requests to trigger preemption
                .collect();

            println!("\nRequest {}: Process {} requests{:?}", i, process_id, request);

            match preempt.allocate(process_id, request) {
                Ok(()) => println!("Allocation successful"),
                Err(e) => println!("Allocation failed: {}", e),
            }
        }
    }

    pub fn run_resource_graph_simulation(&self) {
        println!("\n=== Resource Allocation Graph Simulation ===");

        let mut graph = ResourceAllocationGraph::new();

        // Add processes and resources
        for i in 0..self.state.processes {
            graph.add_process(i);
        }

        for j in 0..self.state.resources {
            graph.add_resource(j);
        }

        for t in 0..self.state.processes {
            for l in 0..self.state.resources {
                if self.state.allocation[t][l] > 0 {
                    graph.add_allocation(l, t);
                }

                if self.state.need[t][l] > 0 {
                    graph.add_request(t, l);
                }
            }
        }

        println!("Graph has cycle: {}", graph.has_cycle());

        if let Some(deadlocked) = graph.detect_deadlock() {
            println!("Deadlock detected in graph! Processes: {:?}", deadlocked);
        } else {
            println!("No deadlock detected in Graph");
        }
    }

    fn print_state(&self, state: &SystemState) {
        println!("Available: {:?}", state.available);

        println!("Allocation matrix:");
        for (i, alloc) in state.allocation.iter().enumerate() {
            println!("  P{}: {:?}", i, alloc);
        }

        println!("Max matrix:");
        for (i, max) in state.max.iter().enumerate() {
            println!("  P{}: {:?}", i, max);
        }

        println!("Need matrix:");
        for (i, need) in state.need.iter().enumerate() {
            println!("  P{}: {:?}", i, need);
        }
    }
}