#[derive(Debug, Clone, Copy)]
pub struct Job  {
  pub id: u32,
  pub arrival_time: u32,
  pub burst_time: u32,
  pub remaining_time: u32,
  pub priority: u8,
  pub start_time: Option<u32>,
  pub completion_time: Option<u32>
}

pub enum JobType {
  Foreground,
  Background,
  HighPriority
}