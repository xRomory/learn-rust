use crate::models::job::Job;

pub trait Scheduler {
  fn add_job(&mut self, job: Job);
  fn schedule(&mut self, current_time: u32) -> Option<Job>;
}