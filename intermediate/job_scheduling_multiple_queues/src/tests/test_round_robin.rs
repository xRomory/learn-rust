#[cfg(test)]
mod tests {
  use crate::{
    models::job::Job,
    scheduler::algorithms::round_robin::RoundRobinScheduler,
    scheduler::scheduler::Scheduler,
  };

  #[test]
  fn test_round_robin() {
    let mut scheduler: RoundRobinScheduler = RoundRobinScheduler::new(2);
    
    scheduler.add_job(Job{
      id: 1,
      burst_time: 5,
      remaining_time: 5,
      arrival_time: 0,
      priority: 0,
      start_time: None,
      completion_time: None
    });

    let result = scheduler.schedule(3);

    println!("Result: {:?}", result);

    assert!(result.is_some());
    assert_eq!(result.unwrap().remaining_time, 3);
  }
}