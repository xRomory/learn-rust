use rand::Rng;
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
        let mut rng = rand::rng();
        let mut requests = Vec::with_capacity(count);

        requests
    }

    fn name(&self) -> String {
        //
    }
}