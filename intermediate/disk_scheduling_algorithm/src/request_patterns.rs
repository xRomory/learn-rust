use rand::{Rng, rngs::ThreadRng};
use crate::{types::{RequestPattern, DiskConfig, DiskRequest}};

#[derive(Debug)]
pub struct ClusteredPattern {
    pub clusters: Vec<(u32, u32)> // (center, spread)
}

impl ClusteredPattern {
    pub fn new(clusters: Vec<(u32, u32)>) -> Self {
        Self { clusters }
    }
}

impl RequestPattern for ClusteredPattern {
    fn generate_requests(
        &self,
        count: usize,
        config: &DiskConfig
    ) -> Vec<DiskRequest> {
        if self.clusters.is_empty() || count == 0 {
            return Vec::new();
        }

        let mut rng: ThreadRng = rand::rng();
        let mut requests = Vec::with_capacity(count);

        for i in 0..count {
            let cluster_index = i % self.clusters.len();
            let (center, spread) = self.clusters[cluster_index];

            let track = if spread > 0 {
                let offset = rng.random_range(1..=spread);
                let direction: i32 = if rng.random_bool(0.5) { 1 } else { -1 };
                ((center as i32) + direction * (offset as i32)).max(0).min(config.max_track as i32) as u32
            } else {
                center
            };

            let arrival = i as u32;
            requests.push(DiskRequest::new(i + 1, track, arrival));
        }

        requests
    }

    fn name(&self) -> String {
        "Clustered".to_string()
    }
}

#[derive(Debug)]
pub struct RandomPattern;

impl RequestPattern for RandomPattern {
    fn generate_requests(
        &self,
        count: usize,
        config: &DiskConfig
    ) -> Vec<DiskRequest> {
        let mut rng: ThreadRng = rand::rng();
        (0..count)
            .map(|i| {
                let track = rng.random_range(0..=config.max_track);
                let arrival = i as u32;
                DiskRequest::new(i + 1, track, arrival)
            })
            .collect()
    }

    fn name(&self) -> String {
        "Random".to_string()
    }
}

#[derive(Debug)]
pub struct SequentialPattern {
    pub start_track: u32,
    pub step: i32 // Can be negative for reverse sequential
}

impl SequentialPattern {
    pub fn new(start_track: u32, step: i32) -> Self {
        Self { start_track, step }
    }
}

impl RequestPattern for SequentialPattern {
    fn generate_requests(
        &self,
        count: usize,
        config: &DiskConfig
    ) -> Vec<DiskRequest> {
        (0..count)
            .map(|i| {
                let track = ((self.start_track as i32) + self.step * (i as i32))
                    .max(0)
                    .min(config.max_track as i32) as u32;
                let arrival = i as u32;
                DiskRequest::new(i + 1, track, arrival)
            })
            .collect()
    }

    fn name(&self) -> String {
        "Sequential".to_string()
    }
}