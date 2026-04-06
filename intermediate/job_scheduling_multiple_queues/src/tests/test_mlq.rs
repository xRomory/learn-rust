#[cfg(test)]
mod tests {
  use crate::{
    queue::{JobQueue, QueueScheduler}, 
    scheduler::{
      algorithms::fcfs::FCFSScheduler
    }
  };

  #[test]
  fn test_basic_fcfs() {
    let queues = vec![
      JobQueue {
        level: 0,
        scheduler: QueueScheduler::FCFS(FCFSScheduler::new())
      }
    ];

    
  }
}