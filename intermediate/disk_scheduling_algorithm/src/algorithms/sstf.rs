use crate::types::{DiskConfig, DiskRequest, DiskScheduler, SimulationResult};

#[derive(Debug)]
pub struct SSTF;

impl DiskScheduler for SSTF {
    fn schedule(&self, requests: &[DiskRequest], config: &DiskConfig) -> SimulationResult {
        let mut result = SimulationResult::new(self.name());

        if requests.is_empty() {
            return result;
        }

        let mut remaining_requests: Vec<DiskRequest> = requests.to_vec();
        let mut current_position = config.initial_position;
        let mut total_seek_time = 0;

        while !remaining_requests.is_empty() {
            let (closest_index, closest_request, seek_distance) = remaining_requests
                .iter()
                .enumerate()
                .map(|(idx, req)| {
                    let distance = current_position.abs_diff(req.track);
                    (idx, req, distance)
                })
                .min_by_key(|&(_, _, distance)| distance)
                .unwrap();
            
            total_seek_time += seek_distance;
            result.seek_sequence.push(closest_request.track);
            current_position = closest_request.track;

            // Remove the processed request
            remaining_requests.remove(closest_index);
        }

        result.total_seek_time = total_seek_time;
        result.requests_processed = requests.len();
        result.average_seek_time = total_seek_time as f64 / requests.len() as f64;

        result
    }

    fn name(&self) -> String {
        "SSTF (Shortest Seek Time First)".to_string()
    }
}

#[derive(Debug)]
pub struct ClusteredSSTF {
    pub cluster_threshold: u32
}

impl ClusteredSSTF {
    pub fn new(cluster_threshold: u32) -> Self {
        Self { cluster_threshold }
    }

    fn form_cluster(
        &self,
        requests: &[DiskRequest],
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
                    current_cluster = vec![request]
                }
            }
        }

        if !current_cluster.is_empty() {
            clusters.push(current_cluster);
        }

        clusters
    }
}

impl DiskScheduler for ClusteredSSTF {
    fn schedule(
        &self,
        requests: &[DiskRequest],
        config: &DiskConfig
    ) -> SimulationResult {
        let mut result = SimulationResult::new(self.name());

        if requests.is_empty() {
            return result;
        }

        let mut current_position = config.initial_position;
        let mut total_seek_time = 0;

        let mut clusters = self.form_cluster(requests);

        while !clusters.is_empty() {
            let (closest_cluster_index, cluster_seek_distance) = clusters
                .iter()
                .enumerate()
                .map(|(idx, cluster)| {
                    let closest_in_cluster = cluster
                        .iter()
                        .map(|req| current_position.abs_diff(req.track))
                        .min()
                        .unwrap();
                    (idx, closest_in_cluster)
                })
                .min_by_key(|&(_, distance)| distance)
                .unwrap();

            let mut cluster = clusters.remove(closest_cluster_index);

            // Process all requests in this cluster using SSTF
            // But now we should track the seek distance to the cluster
            total_seek_time += cluster_seek_distance;

            // Find and process the closest request in the cluster first
            let (closest_index, closest_request, _) = cluster
                .iter()
                .enumerate()
                .map(|(idx, req)| {
                    let distance = current_position.abs_diff(req.track);
                    (idx, req, distance)
                })
                .min_by_key(|&(_, _, distance)| distance)
                .unwrap();

            result.seek_sequence.push(closest_request.track);
            current_position = closest_request.track;
            cluster.remove(closest_index);

            // Process remaining requests in this cluster using SSTF
            while !cluster.is_empty() {
                let(closest_index, closest_request, seek_distance) = cluster
                    .iter()
                    .enumerate()
                    .map(|(idx, req)| {
                        let distance = current_position.abs_diff(req.track);
                        (idx, req, distance)
                    })
                    .min_by_key(|&(_, _, distance)| distance)
                    .unwrap();

                total_seek_time += seek_distance;
                result.seek_sequence.push(closest_request.track);
                current_position = closest_request.track;
                cluster.remove(closest_index);
            }
        }

        result.total_seek_time = total_seek_time;
        result.requests_processed = requests.len();
        result.average_seek_time = total_seek_time as f64 / requests.len() as f64;

        result
    }

    fn name(&self) -> String {
        format!("SSTF (Clustered, threshold: {})", self.cluster_threshold)
    }
}