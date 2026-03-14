use std::collections::VecDeque;
use crate::models::job::Job;

pub struct JobQueue {
  pub level: u8,
  pub time_slice: Option<u32>,
  pub queue: VecDeque<Job>
}

impl JobQueue {
  pub fn new(
    level: u8,
    time_slice: Option<u32>,
  ) -> Self {
    Self {
      level,
      time_slice,
      queue: VecDeque::new()
    }
  }

  pub fn enqueue(&mut self, job: Job) {
    self.queue.push_back(job);
  }

  pub fn dequeue(&mut self) -> Option<Job> {
    self.queue.pop_front()
  }

  pub fn job_is_empty(&mut self) -> bool {
    self.queue.is_empty()
  }
}