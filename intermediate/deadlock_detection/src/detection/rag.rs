use std::collections::{HashMap, HashSet};

use crate::detection::DeadlockDetector;

#[derive(Debug, Clone)]
pub struct ResourceAllocationGraph {
    processes: HashSet<usize>,
    resources: HashSet<usize>,
    allocation_edges: HashMap<usize, Vec<usize>>,
    request_edges: HashMap<usize, Vec<usize>>
}

impl ResourceAllocationGraph {
    pub fn new() -> Self {
        Self { 
            processes: HashSet::new(), 
            resources: HashSet::new(), 
            allocation_edges: HashMap::new(), 
            request_edges: HashMap::new() 
        }
    }

    pub fn add_process(&mut self, process_id: usize) {
        self.processes.insert(process_id);
    }

    pub fn add_resource(&mut self, resource_id: usize) {
        self.resources.insert(resource_id);
    }

    pub fn add_allocation(
        &mut self, 
        resource_id: usize, 
        process_id: usize
    ) {
        self.allocation_edges
            .entry(resource_id)
            .or_insert_with(Vec::new)
            .push(process_id);
    }

    pub fn add_request(
        &mut self,
        process_id: usize,
        resource_id: usize
    ) {
        self.request_edges
            .entry(process_id)
            .or_insert_with(Vec::new)
            .push(resource_id);
    }

    pub fn has_cycle(&self) -> bool {
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();

        for &process in &self.processes {
            if !visited.contains(&process) {
                if self.detect_cycle(
                    process, 
                    &mut visited, 
                    &mut recursion_stack
                ) {
                    return true;
                }
            }
        }

        false
    }

    fn detect_cycle(
        &self,
        node: usize,
        visited: &mut HashSet<usize>,
        recursion_stack: &mut HashSet<usize>
    ) -> bool {
        visited.insert(node);
        recursion_stack.insert(node);

        for next in self.successors(node) {
            if !visited.contains(&next) {
                if self.detect_cycle(next, visited, recursion_stack) {
                    return true;
                }
            } else if recursion_stack.contains(&next) {
                return true;
            }
        }

        recursion_stack.remove(&node);
        false
    }

    // Returns all processes reachable from a process through request → allocation edges.
    fn successors(&self, node: usize) -> Vec<usize> {
        self.request_edges
            .get(&node)
            .into_iter()
            .flatten()  // resources
            .flat_map(|resource| {
                self.allocation_edges
                    .get(resource)
                    .into_iter()
                    .flatten() // processes waiting on those resources
                    .copied()
            })
            .collect()
    }
}

impl DeadlockDetector for ResourceAllocationGraph {
    fn detect_deadlock(&self) -> Option<Vec<usize>> {
        if self.has_cycle() {
            Some(self.processes.iter().copied().collect())
        } else {
            None
        }
    }

    fn is_safe_state(&self) -> bool {
        !self.has_cycle()
    }
}