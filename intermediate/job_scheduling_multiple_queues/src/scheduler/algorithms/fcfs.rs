use crate::{models::job::Job, scheduler::scheduler::Scheduler};

pub struct FCFSScheduler {
  pub jobs: Vec<Job>
}

impl FCFSScheduler {
  pub fn new() -> Self {
    FCFSScheduler { jobs: Vec::new() }
  }
}

impl Scheduler for FCFSScheduler {
  fn add_job(&mut self, job: Job) {
    self.jobs.push(job);
  }

  fn schedule(&mut self, current_time: u32) -> Option<Job> {
    if self.jobs.is_empty() {
      return None;
    }

    let idx = self.jobs
      .iter()
      .enumerate()
      .filter(|(_, job)| job.arrival_time <= current_time)
      .min_by_key(|(_, job)| job.arrival_time)
      .map(|(i, _)| i);

    if let Some(index) = idx {
      let mut job = self.jobs.remove(index);

      job.start_time = Some(current_time);
      job.completion_time = Some(current_time + job.burst_time);

      return Some(job);
    }

    None
  }
}