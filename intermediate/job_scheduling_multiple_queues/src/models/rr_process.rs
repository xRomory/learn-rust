use crate::models::base_process::BaseProcess;

#[derive(Debug)]
pub struct RoundRobinProcess {
  pub base: BaseProcess,
  pub remaining_time: u32,
}

impl RoundRobinProcess {
  pub fn new(
    pid: usize,
    arrival_time: u32,
    burst_time: u32,
  ) -> Self {
    RoundRobinProcess { 
      base: BaseProcess::new(
        pid,
        arrival_time,
        burst_time,
      ),
      remaining_time: burst_time,
    }
  }
}