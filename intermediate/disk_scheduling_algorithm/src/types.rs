use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiskRequest {
    pub request_id: usize,
    pub track: u32,
    pub arrival_time: u32
}

impl DiskRequest {
    pub fn new(request_id: usize, track: u32, arrival_time: u32) -> Self {
        Self {
            request_id,
            track,
            arrival_time
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub total_seek_time: u32,
    pub average_seek_time: f64,
    pub requests_processed: usize,
    pub seek_sequence: Vec<u32>,
    pub algorithm_name: String,
}

impl SimulationResult {
    pub fn new(algorithm_name: String) -> Self {
        Self {
            total_seek_time: 0,
            average_seek_time: 0.0,
            requests_processed: 0,
            seek_sequence: Vec::new(),
            algorithm_name
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiskConfig {
    pub initial_position: u32,
    pub total_tracks: u32,
    pub max_track: u32
}

impl Default for DiskConfig {
    fn default() -> Self {
        Self {
            initial_position: 50,
            total_tracks: 200,
            max_track: 199,
        }
    }
}

pub trait DiskScheduler: fmt::Debug {
    fn schedule(&self, requests: &[DiskRequest], config: &DiskConfig) -> SimulationResult;

    fn name(&self) -> String;
}

// Trait for request pattern generators
pub trait RequestPattern {
    fn generate_requests(&self, count: usize, config: &DiskConfig) -> Vec<DiskRequest>;
    fn name(&self) -> String;
}