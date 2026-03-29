#[cfg(test)]
mod tests {
  use crate::{
    models::job::Job, 
    scheduler::{
      algorithms::hrrn::HRRNScheduler, 
      scheduler::Scheduler
    }
  };

  #[test]
  fn test_hrrn_scheduling_basic() {
    let mut scheduler = HRRNScheduler::new();

    scheduler.add_job(Job { 
      id: 1,
      arrival_time: 0,
      burst_time: 4,
      remaining_time: 4,
      priority: 5,
      start_time: None,
      completion_time: None,
    });

    scheduler.add_job(create_job(1, 0, 5));
    scheduler.add_job(create_job(2, 1, 3));
    scheduler.add_job(create_job(3, 2, 2));

    let job = 
      scheduler.schedule(4).expect("Should schedule a job");
    assert!(job.id == 1 || job.id == 2, "Scheduled job should be one of the added jobs");
    assert!(job.start_time.is_some(), "Job should have a start time");
    assert!(job.completion_time.is_some(), "Job should have a completion time");

    let job2 = 
      scheduler.schedule(7).expect("Should schedule the second job");
    assert_ne!(job.id, job2.id, "Should schedule the other job next");

    // Print jobs
    println!("Job1: {:?}", job);
    println!("Job2: {:?}", job2);
  }

  #[test]
  fn test_hrrn_empty_scheduler() {
    let mut scheduler = HRRNScheduler::new();

    let result = scheduler.schedule(0);
    println!("Should return None: {:?}", result);
    assert!(result.is_none(), "Should return None when no jobs are present");
  }

  // Helper function to create job
  fn create_job(
    id: u32,
    arrival_time: u32,
    burst_time: u32,

  ) -> Job {
    Job {
      id,
      arrival_time,
      burst_time,
      remaining_time: burst_time, 
      priority: 0,
      start_time: None,
      completion_time: None
    }
  }
}