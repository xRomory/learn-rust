#[derive(Debug)]
pub struct BaseProcess {
  pub pid: usize,
  pub arrival_time: u32,
  pub burst_time: u32,
  pub turnaround_time: u32,
  pub completion_time: u32,
  pub waiting_time: u32,
}

impl BaseProcess {
  pub fn new(
    pid: usize,
    arrival_time: u32,
    burst_time: u32,
  ) -> Self {
    BaseProcess {
      pid,
      arrival_time,
      burst_time,
      turnaround_time: 0,
      completion_time: 0,
      waiting_time: 0
    }
  }
}