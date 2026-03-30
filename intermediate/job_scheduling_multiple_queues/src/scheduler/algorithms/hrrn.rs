use crate::{
  models::job::Job,
  scheduler::scheduler::Scheduler
};

pub struct HRRNScheduler {
  pub jobs: Vec<Job>
}

impl HRRNScheduler {
  pub fn new() -> Self {
    HRRNScheduler { jobs: Vec::new() }
  }

  pub fn calculate_response_ratio(job: &Job, current_time: u32) -> f64 {
    let waiting_time: u32 = if current_time > job.arrival_time {
      current_time.saturating_sub(job.arrival_time)
    } else {
      0
    };

    (waiting_time as f64 + job.burst_time as f64) / job.burst_time as f64
  }
}

impl Scheduler for HRRNScheduler {
  fn add_job(&mut self, job: Job) {
    self.jobs.push(job);
  }

  fn schedule(&mut self, current_time: u32) -> Option<Job> {
    if self.jobs.is_empty() {
      return None;
    }

    let mut idx: Option<usize> = None;
    let mut highest_rr = f64::MIN;

    for (i, job) in self.jobs.iter().enumerate() {
      if job.arrival_time > current_time {
        continue;
      }

      let response_ratio = Self::calculate_response_ratio(job, current_time);
    
      if response_ratio > highest_rr {
        highest_rr = response_ratio;
        idx = Some(i);
      }
    }

    if let Some(index) = idx {
      let mut job = self.jobs.remove(index);

      job.start_time = Some(current_time);
      job.completion_time = Some(current_time + job.burst_time);

      return Some(job);
    }

    None
  }
}