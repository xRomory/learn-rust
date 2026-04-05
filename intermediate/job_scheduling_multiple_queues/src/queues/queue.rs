use crate::models::job::Job;
use crate::scheduler::algorithms;
use crate::scheduler::scheduler::Scheduler;

pub enum QueueScheduler {
  RoundRobin(algorithms::round_robin::RoundRobinScheduler),
  FCFS(algorithms::fcfs::FCFSScheduler),
}

pub struct JobQueue {
  pub level: u8,
  pub scheduler: QueueScheduler
}

impl JobQueue {
  pub fn add_job(&mut self, job: Job) {
    match &mut self.scheduler {
      QueueScheduler::RoundRobin(sched) => sched.add_job(job),
      QueueScheduler::FCFS(sched) => sched.add_job(job),
    }
  }

  pub fn schedule(
    &mut self,
    current_time: u32,
  ) -> Option<Job> {
    match &mut self.scheduler {
      QueueScheduler::RoundRobin(sched) => sched.schedule(current_time),
      QueueScheduler::FCFS(sched) => sched.schedule(current_time),
    }
  }

  pub fn job_is_empty(&self) -> bool {
    match &self.scheduler {
      QueueScheduler::RoundRobin(sched) => sched.queue.is_empty(),
      QueueScheduler::FCFS(sched) => sched.jobs.is_empty(),
    }
  }
}