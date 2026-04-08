#[cfg(test)]
mod tests {
  use crate::{
    job::JobType,
    scheduler::scheduler::Scheduler,
    tests::helper::create_job::{create_job_mlq, create_mlq}
  };

  #[test]
  fn test_mlq_scheduling_order() {

    let mut mlq = create_mlq();

    let job1 = create_job_mlq(1, 0, 4, JobType::Foreground, 0);
    let job2 = create_job_mlq(2, 0, 3, JobType::Background, 1);

    mlq.add_job(job1);
    mlq.add_job(job2);

    let result1 = mlq.schedule(0).unwrap();
    // println!("First Result: {:?}", result1);
    assert_eq!(result1.id, 1);

    let result2 = mlq.schedule(2).unwrap();
    // println!("Second Result: {:?}", result2);
    assert_eq!(result2.id, 1);

    let result3 = mlq.schedule(4).unwrap(); // Now FCFS is scheduled, since RR queue is now empty
    // println!("Third Result: {:?}", result3);
    assert_eq!(result3.id, 2);
  }

  #[test]
  fn test_mlq_fallback_to_lower_queue() {
    let mut scheduler = create_mlq();

    scheduler.add_job(create_job_mlq(1, 0, 5, JobType::Background, 1));

    let job = scheduler.schedule(0).unwrap();
    println!("MLQ: {:?}", job);
    assert_eq!(job.id, 1);
  }

  #[test]
  fn test_mlq_round_robin_behavior() {
    let mut scheduler = create_mlq();

    scheduler.add_job(create_job_mlq(1, 0, 5, JobType::Foreground, 0));

    let job1 = scheduler.schedule(0).unwrap();
    // println!("RT: {:?}", job1.remaining_time);
    assert_eq!(job1.remaining_time, 3);

    let job2 = scheduler.schedule(2).unwrap();
    // println!("RT2: {:?}", job2.remaining_time);
    assert_eq!(job2.remaining_time, 1);
  }

  #[test]
  fn test_mlq_respects_arrival_time() {
    let mut scheduler = create_mlq();

    // High priority but NOT arrived
    scheduler.add_job(create_job_mlq(1, 5, 3, JobType::Foreground, 0));

    // Lower priority but arrived
    scheduler.add_job(create_job_mlq(2, 0, 3, JobType::Background, 1));

    let job = scheduler.schedule(0).unwrap();
    // println!("Job: {:?}", job);
    assert_eq!(job.id, 2);
  }
}