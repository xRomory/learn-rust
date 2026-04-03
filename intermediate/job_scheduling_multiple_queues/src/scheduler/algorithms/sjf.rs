use crate::{
  models::job::Job,
  scheduler::scheduler::Scheduler
};

pub struct SJFScheduler {
  pub jobs: Vec<Job>
}

impl SJFScheduler {
  pub fn new() -> Self {
    Self { jobs: Vec::new() }
  }
}

impl Scheduler for SJFScheduler {
  fn add_job(&mut self, job: Job) {
    self.jobs.push(job);
  }

  fn schedule(&mut self, current_time: u32) -> Option<Job> {
    // Filter arrived jobs, then select the shortest burst time
    let index_shortest_bt = self.jobs
      .iter()
      .enumerate()
      .filter(|(_, job)| job.arrival_time <= current_time)
      .min_by_key(|(_, job)| job.burst_time)
      .map(|(i, _)| i);

    if let Some(index) = index_shortest_bt {
      let mut job = self.jobs.remove(index);

      job.start_time = Some(current_time);
      job.completion_time = Some(current_time + job.burst_time);

      return Some(job);
    }

    None
  }
}