use crate::{
  queues::queue::JobQueue,
  scheduler::scheduler::Scheduler,
  models::job::Job
};

pub struct MLQScheduler {
  pub queues: Vec<JobQueue>
}

impl Scheduler for MLQScheduler {
  fn add_job(&mut self, job: Job) {
    let level = job.priority as usize;
    self.queues[level].add_job(job);
  }

  fn schedule(
    &mut self,
    current_time: u32
  ) -> Option<Job> {
    

    for queue in &mut self.queues {
      if queue.job_is_empty() {
        continue;
      }
      
      if let Some(job) = queue.schedule(current_time) {
        return Some(job)
      }
    }

    None
  }
}