use crate::models::rr_process::RoundRobinProcess;

pub struct RoundRobinScheduler {
  pub processes: Vec<RoundRobinProcess>,
  pub time_quantum: u32,
}

impl RoundRobinScheduler {
  pub fn new(time_quantum: u32) -> Self {
    RoundRobinScheduler { 
      processes: Vec::new(), 
      time_quantum
    }
  }
}