use crate::models::job::Job;

pub fn create_job(
  id: u32,
  arrival_time: u32,
  burst_time: u32,
) -> Job {
  Job {
    id,
    arrival_time,
    burst_time,
    remaining_time: burst_time, 
    priority: 0,
    start_time: None,
    completion_time: None
  }
}