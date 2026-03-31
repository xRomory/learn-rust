#[cfg(test)]
mod tests {
  use crate::{
    scheduler::{
      algorithms::round_robin::RoundRobinScheduler,
      scheduler::Scheduler
    },
    tests::helper::create_job::create_job,
  };

  #[test]
  fn test_round_robin() {
    let mut scheduler: RoundRobinScheduler = RoundRobinScheduler::new(2);

    scheduler.add_job(create_job(1, 0, 5));

    let result = scheduler.schedule(3);

    println!("Result: {:?}", result);

    assert!(result.is_some());
    assert_eq!(result.unwrap().remaining_time, 3);
  }
}