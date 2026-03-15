use crate::{
  queues::queue::JobQueue,
  scheduler::scheduler::Scheduler
};

pub struct MLQScheduler {
  pub queues: Vec<JobQueue>
}

impl Scheduler for MLQScheduler {
  fn add_job(&mut self, job: crate::job::Job) {
    let level = job.priority as usize;
    self.queues[level].enqueue(job);
  }

  fn schedule(
    &mut self,
    _current_time: u32
  ) -> Option<crate::job::Job> {
    for queue in &mut self.queues {
      if !queue.job_is_empty() {
        return queue.dequeue();
      }
    }

    None
  }
}