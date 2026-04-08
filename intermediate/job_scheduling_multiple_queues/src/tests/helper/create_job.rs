use crate::{
  models::job::{Job, JobType},
  queues::queue::{JobQueue, QueueScheduler},
  scheduler::{
    mlq::MLQScheduler,
    algorithms::{
      fcfs::FCFSScheduler,
      round_robin::RoundRobinScheduler,
    }
  }
};

pub fn create_job(
  id: u32,
  arrival_time: u32,
  burst_time: u32,
  job_type: JobType
) -> Job {
  Job {
    id,
    arrival_time,
    burst_time,
    remaining_time: burst_time, 
    priority: 0,
    job_type,
    start_time: None,
    completion_time: None
  }
}

pub fn create_job_mlq(
  id: u32,
  arrival_time: u32,
  burst_time: u32,
  job_type: JobType,
  priority: u8
) -> Job {
  Job {
    id,
    arrival_time,
    burst_time,
    remaining_time: burst_time, 
    priority,
    job_type,
    start_time: None,
    completion_time: None
  }
}

pub fn create_mlq() -> MLQScheduler {
  MLQScheduler {
    queues: vec![
      JobQueue {
        level: 0,
        scheduler: QueueScheduler::RoundRobin(RoundRobinScheduler::new(2))
      },
      JobQueue {
        level: 1,
        scheduler: QueueScheduler::FCFS(FCFSScheduler::new())
      }
    ],
  }
}