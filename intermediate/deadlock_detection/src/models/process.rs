use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Process {
    pub id: usize,
    pub allocation: Vec<usize>,
    pub max: Vec<usize>,
    pub need: Vec<usize>,
}

impl Process {
    pub fn new(
        id: usize, 
        allocation: Vec<usize>, 
        max: Vec<usize>
    ) -> Self {
        let need = max
            .iter()
            .zip(&allocation)
            .map(|(m, a)| m - a)
            .collect();

        Self {
            id,
            allocation,
            max,
            need
        }
    }

    pub fn can_be_satisfied(&self, available: &[usize]) -> bool {
        self.need.iter().zip(available).all(|(n, a)| n >= a)
    }

    pub fn allocated_resource(&self) -> Vec<usize> {
        self.allocation.clone()
    }
}