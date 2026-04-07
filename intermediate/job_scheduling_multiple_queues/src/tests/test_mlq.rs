#[cfg(test)]
mod tests {
  use crate::{
    job::JobType,
    queue::{
      JobQueue,
      QueueScheduler
    },
    scheduler::{
      algorithms::{fcfs::FCFSScheduler, round_robin::RoundRobinScheduler},
      mlq::MLQScheduler, scheduler::Scheduler
    },
    tests::helper::create_job::create_job_mlq
  };

  #[test]
  fn test_mlq_scheduling_order() {
    let rr_queue = JobQueue {
      level: 0,
      scheduler: QueueScheduler::RoundRobin(RoundRobinScheduler::new(2))
    };

    let fcfs_queue = JobQueue {
      level: 1,
      scheduler: QueueScheduler::FCFS(FCFSScheduler::new())
    };

    let mut mlq = MLQScheduler { queues: vec![rr_queue, fcfs_queue] };

    let job1 = create_job_mlq(1, 0, 4, JobType::Foreground, 0);
    let job2 = create_job_mlq(2, 0, 3, JobType::Background, 1);

    mlq.add_job(job1);
    mlq.add_job(job2);

    let result1 = mlq.schedule(0).unwrap();
    println!("First Result: {:?}", result1);
    assert_eq!(result1.id, 1);

    let result2 = mlq.schedule(2).unwrap();
    println!("Second Result: {:?}", result2);
    assert_eq!(result2.id, 1);

    let result3 = mlq.schedule(4).unwrap(); // Now FCFS is scheduled, since RR queue is now empty
    println!("Third Result: {:?}", result3);
    assert_eq!(result3.id, 2);
  }
}