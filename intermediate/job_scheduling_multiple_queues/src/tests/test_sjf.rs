#[cfg(test)]
mod tests {
  use crate::{
    scheduler::{
      algorithms::sjf::SJFScheduler, 
      scheduler::Scheduler
    }, 
    tests::helper::create_job::create_job
  };

  #[test]
  fn test_sjf_basic() {
    let mut scheduler = SJFScheduler::new();

    scheduler.add_job(create_job(1, 0, 5));
    scheduler.add_job(create_job(2, 2, 1));
    scheduler.add_job(create_job(3, 3, 4));

    let job = scheduler.schedule(3).unwrap();
    // let job2 = scheduler.schedule(3).unwrap();
    // let job3 = scheduler.schedule(3).unwrap();
    // println!("SJF Job1: {:?}", job);
    // println!("SJF Job2: {:?}", job2);
    // println!("SJF Job3: {:?}", job3);

    // Job ID: 2 have the shortest burst time
    assert_eq!(job.id, 2);
  }

  #[test]
  fn sjf_no_arrived_jobs() {
    let mut scheduler = SJFScheduler::new();

    scheduler.add_job(create_job(1, 5, 2));

    let result = scheduler.schedule(0);
    // println!("Result: {:?}", result);
    assert!(result.is_none());
  }

  #[test]
  fn test_sjf_arrival_filtering() {
    let mut scheduler = SJFScheduler::new();

    scheduler.add_job(create_job(1, 5, 1));
    scheduler.add_job(create_job(2, 0, 3));

    let job = scheduler.schedule(6).unwrap();
    // println!("Job Filtered: {:?}", job);

    // Only one available and Job ID 1 have the shortest bt
    assert_eq!(job.id, 1);
  }

  #[test]
  fn test_sjf_completion_time() {
    let mut scheduler = SJFScheduler::new();

    scheduler.add_job(create_job(1, 5, 1));

    let job = scheduler.schedule(5).unwrap();
    // println!("Job Completion Time: {:?}", job.completion_time);
    assert_eq!(job.start_time, Some(5));  // Should have 5
    assert_eq!(job.completion_time, Some(6));  // Should have 6 (AT + CT)

  }
}