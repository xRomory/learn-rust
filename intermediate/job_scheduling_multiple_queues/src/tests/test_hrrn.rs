#[cfg(test)]
mod tests {
  use crate::{ 
    scheduler::{
      algorithms::hrrn::HRRNScheduler, 
      scheduler::Scheduler
    },
    models::job::JobType,
    tests::helper::create_job::create_job
  };

  #[test]
  fn test_response_ratio_formula() {
    let job = create_job(1, 0, 5, JobType::Background);

    let response_ratio =  HRRNScheduler::calculate_response_ratio(&job, 5);

    // Should get 2.0 since (5 + 5) / 5 = 2.0
    // println!("Response Ratio: {:?}", response_ratio);
    assert_eq!(response_ratio, 2.0);
  }

  #[test]
  fn test_hrrn_scheduling_basic() {
    let mut scheduler = HRRNScheduler::new();

    let jobs = vec![
      create_job(1, 0, 5, JobType::Background),
      create_job(2, 1, 3, JobType::Background),
      create_job(3, 2, 2, JobType::Background),
    ];

    let current_time = 4;

    for job in &jobs {
      let rr = HRRNScheduler::calculate_response_ratio(job, current_time);
      scheduler.add_job(*job);
      println!(
        "Job {} → RR = {:.2}",
        job.id,
        rr
      );
    }

    let job = 
      scheduler.schedule(4).expect("Should schedule a job");

    println!("Job 1: {:?}", job);
    
    assert_eq!(job.id, 2);
    assert!(job.start_time.is_some(), "Job should have a start time");
    assert!(job.completion_time.is_some(), "Job should have a completion time");

    let job2 = 
      scheduler.schedule(7).expect("Should schedule the second job");
    
    println!("Job 2: {:?}", job2);
    assert_ne!(job.id, job2.id, "Should schedule the other job next");
    assert_eq!(job2.id, 3);
  }

  #[test]
  fn test_hrrn_empty_scheduler() {
    let mut scheduler = HRRNScheduler::new();

    let result = scheduler.schedule(0);
    assert!(result.is_none(), "Should return None when no jobs are present");
  }

  #[test]
  fn test_hrrn_no_arrived_jobs() {
    let mut scheduler = HRRNScheduler::new();

    scheduler.add_job(create_job(1, 10, 2, JobType::Background));

    let result = scheduler.schedule(0);
    assert!(result.is_none(), "Should return None if no jobs have arrived yet");
  }
}