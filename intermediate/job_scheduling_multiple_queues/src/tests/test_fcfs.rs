#[cfg(test)]
mod tests {
  use crate::{
    scheduler::{
      algorithms::fcfs::FCFSScheduler,
      scheduler::Scheduler
    }, 
    tests::helper::create_job::create_job
  };

  #[test]
  fn test_fcfs_basic(){
    let mut scheduler = FCFSScheduler::new();

    scheduler.add_job(create_job(1, 2, 4));
    scheduler.add_job(create_job(2, 0, 2));
    scheduler.add_job(create_job(3, 1, 5));

    let job = 
      scheduler.schedule(2).expect("Should schedule a job");

    println!("Job ID should be 2: {:?}", job);
    assert_eq!(job.id, 2);
  }

  #[test]
  fn test_no_arrived_jobs() {
    let mut scheduler = FCFSScheduler::new();
    scheduler.add_job(create_job(1, 2, 4));
    
    let result = scheduler.schedule(0);

    println!("Result should be None: {:?}", result);
    assert!(result.is_none());
  }

  #[test]
  fn test_fcfs_completion_time() {
    let mut scheduler = FCFSScheduler::new();

    scheduler.add_job(create_job(1, 9, 15));

    let job =
      scheduler.schedule(9).unwrap();
    
    println!("Start time should be 9: {:?}", job.start_time);
    println!("Completion time should be 24: {:?}", job.completion_time);
    assert_eq!(job.start_time, Some(9));
    assert_eq!(job.completion_time, Some(24));
  }
}