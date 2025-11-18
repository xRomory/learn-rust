use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SystemState {
    pub available: Vec<usize>,
    pub allocation: Vec<Vec<usize>>,
    pub max: Vec<Vec<usize>>,
    pub need: Vec<Vec<usize>>,
    pub processes: usize,
    pub resources: usize
}

impl SystemState {
    pub fn new(
        available: Vec<usize>,
        max: Vec<Vec<usize>>,
        allocation: Vec<Vec<usize>>
    ) -> Self {
        let processes = max.len();
        let resources = available.len();

        let mut need = vec![vec![0; resources]; processes];

        for i in 0..processes {
            for j in 0..resources {
                need[i][j] = max[i][j] - allocation[i][j];
            }
        }

        Self {
            available,
            allocation,
            max,
            need,
            processes,
            resources
        }
    }

    pub fn calculate_need(&mut self) {
        for i in 0..self.processes {
            for j in 0..self.resources {
                self.need[i][j] = self.max[i][j] - self.allocation[i][j];
            }
        }
    }

    pub fn is_safe_state(&self) -> bool {
        let mut work = self.available.clone();
        let mut finish = vec![false; self.processes];

        for _ in 0..self.processes {
            let mut found = false;

            for i in 0..self.processes {
                if !finish[i] && self.can_allocate(i, &work) {
                    // Process can be allocated resources
                    for j in 0..self.resources {
                        work[j] += self.allocation[i][j];
                    }

                    finish[i] = true;
                    found = true;
                }
            }

            if !found {
                break;
            }
        }

        finish.iter().all(|&f| f)
    }

    fn can_allocate(
        &self,
        process: usize,
        work: &[usize],
    ) -> bool {
        (0..self.resources).all(|j| self.need[process][j] <= work[j])
    }
}