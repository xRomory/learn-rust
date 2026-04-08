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
    let mut idx: usize = 0;
    while idx < self.queue.len() {
      if self.queue[idx].arrival_time <= current_time {
        let mut job = self.queue.remove(idx).unwrap();
        let run_time = job.remaining_time.min(self.time_quantum);
        job.remaining_time -= run_time;

        if job.remaining_time > 0 {
          self.queue.push_back(job);
        } else {
          job.completion_time = Some(current_time + run_time);
        }

        return Some(job)
      } else {
        idx += 1;
      }
    }

    None
  }
}