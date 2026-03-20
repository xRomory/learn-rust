use crate::models::base_process::BaseProcess;

#[derive(Debug)]
pub struct HRRNProcess {
  pub base: BaseProcess,
  pub response_ratio: u32,
}

impl HRRNProcess {
  pub fn new(
    pid: usize,
    arrival_time: u32,
    burst_time: u32,
  ) -> Self {
    HRRNProcess { 
      base: BaseProcess::new(
        pid,
        arrival_time,
        burst_time
      ), 
      response_ratio: 0,
    }
  }

  pub fn calculate_response_ratio(
    &self,
    current_time: u32
  ) -> f64 {
    let waiting_time = if current_time > self.base.arrival_time {
      current_time - self.base.arrival_time
    } else {
      0
    };

    // Response Ratio formula
    (waiting_time as f64 + self.base.burst_time as f64) / self.base.burst_time as f64
  }
}