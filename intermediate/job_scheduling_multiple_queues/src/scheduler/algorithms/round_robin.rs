use std::collections::VecDeque;

use crate::{
  models::job::Job, 
  scheduler::scheduler::Scheduler
};

pub struct RoundRobinScheduler {
  pub queue: VecDeque<Job>,
  pub time_quantum: u32,
}

impl RoundRobinScheduler {
  pub fn new(time_quantum: u32) -> Self {
    RoundRobinScheduler { 
      queue: VecDeque::new(), 
      time_quantum
    }
  }
}

impl Scheduler for RoundRobinScheduler {
  fn add_job(&mut self, job: Job) {
    self.queue.push_back(job);
  }

  fn schedule(
    &mut self,
    current_time: u32
  ) -> Option<Job> {
    if let Some(mut job) = self.queue.pop_front() {
      let run_time = job.remaining_time.min(self.time_quantum);
      job.remaining_time -= run_time;

      if job.remaining_time > 0 {
        self.queue.push_back(job);
      } else {
        job.completion_time = Some(current_time + run_time);
      }

      Some(job)
    } else {
      None
    }
  }
}