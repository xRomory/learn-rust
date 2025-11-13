use crate::types::{DiskConfig, DiskRequest, DiskScheduler, SimulationResult};

#[derive(Debug)]
pub struct BasicFCFS;

impl DiskScheduler for BasicFCFS {
    fn schedule(&self, requests: &[DiskRequest], config: &DiskConfig) -> SimulationResult {
        let mut result = SimulationResult::new(self.name());
        let mut current_position = config.initial_position;

        for request in requests {
            let seek_distance = current_position.abs_diff(request.track);

            result.total_seek_time += seek_distance;
            result.seek_sequence.push(request.track);
            current_position = request.track;
        }

        result.requests_processed = requests.len();
        result.average_seek_time = if !requests.is_empty() {
            result.total_seek_time as f64 / requests.len() as f64
        } else {
            0.0
        };

        result
    }

    fn name(&self) -> String {
        "Basic FCFS Algorithm".to_string()
    }
}

#[derive(Debug)]
pub struct ClusteredFCFS {
    pub cluster_threshold: u32,
}

impl ClusteredFCFS {
    pub fn new(cluster_threshold: u32) -> Self {
        Self { cluster_threshold }
    }

    fn form_clusters(
        &self,
        requests: &[DiskRequest]
    ) -> Vec<Vec<DiskRequest>> {
        if requests.is_empty() {
            return Vec::new();
        }

        let mut sorted_requests = requests.to_vec();
        sorted_requests.sort_by_key(|r| r.track);

        let mut clusters = Vec::new();
        let mut current_cluster = Vec::new();

        for request in sorted_requests {
            if current_cluster.is_empty() {
                current_cluster.push(request);
            } else {
                let last_track = current_cluster.last().unwrap().track;

                if request.track.abs_diff(last_track) <= self.cluster_threshold {
                    current_cluster.push(request);
                } else {
                    clusters.push(current_cluster);
                    current_cluster = vec![request];
                }
            }
        }

        if !current_cluster.is_empty() {
            clusters.push(current_cluster);
        }

        clusters
    }
}

impl DiskScheduler for ClusteredFCFS {
    fn schedule(
        &self,
        requests: &[DiskRequest],
        config: &DiskConfig
    ) -> SimulationResult {
        let mut result = SimulationResult::new(self.name());
        let mut current_position = config.initial_position;

        let clusters = self.form_clusters(requests);

        for cluster in clusters {
            let mut cluster_requests = cluster;
            cluster_requests.sort_by_key(|r| r.arrival_time);

            for request in cluster_requests {
                let seek_distance = current_position.abs_diff(request.track);
                result.total_seek_time += seek_distance;
                result.seek_sequence.push(request.track);
                current_position = request.track;
            }
        }

        result.requests_processed = requests.len();
        result.average_seek_time = if !requests.is_empty() {
            result.total_seek_time as f64 / requests.len() as f64
        } else {
            0.0
        };

        result
    }

    fn name(&self) -> String {
        "Clustered FCFS".to_string()
    }
}