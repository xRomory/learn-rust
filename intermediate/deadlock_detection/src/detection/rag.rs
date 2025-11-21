use std::collections::{HashMap, HashSet};

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

    fn detect_cycle(
        &self,
        node: usize,
        visited: &mut HashSet<usize>,
        recursion_stack: &mut HashSet<usize>
    ) -> bool {
        visited.insert(node);
        recursion_stack.insert(node);

        if let Some(resources) = self.request_edges.get(&node) {
            for &resource in resources {
                if let Some(processes) = self.allocation_edges.get(&resource) {
                    for &next_process in processes {
                        if !visited.contains(&next_process) {
                            if self.detect_cycle(next_process, visited, recursion_stack) {
                                return true;
                            }
                        } else if recursion_stack.contains(&next_process) {
                            return true;
                        }
                    }
                }
            }
        }

        recursion_stack.remove(&node);
        false
    }
}