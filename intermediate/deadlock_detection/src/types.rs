use std::collections::HashMap;

pub type ProcessId = usize;
pub type ResouceId = usize;
pub type ResourceCount = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Process {
    pub id: ProcessId,
    pub allocated: HashMap<ResouceId, ResourceCount>,
    pub maximum: HashMap<ResouceId, ResourceCount>,
    pub needed: HashMap<ResouceId, ResourceCount>
}

impl Process {
    pub fn new(
        id: ProcessId, 
        maximum: HashMap<ResouceId, ResourceCount>
    ) -> Self {
        let needed = maximum.clone();

        Self {
            id,
            allocated: HashMap::new(),
            maximum,
            needed,
        }
    }

    pub fn request_resources(
        &mut self,
        resources: &HashMap<ResouceId, ResourceCount>
    ) {
        for (&resource_id, &resource_count) in resources {
            if let Some(allocated) = self.allocated.get_mut(&resource_id) {
                *allocated += resource_count;
            } else {
                self.allocated.insert(resource_id, resource_count);
            }

            if let Some(needed) = self.needed.get_mut(&resource_id) {
                *needed -= resource_count;
            }
        }
    }

    pub fn release_resources(
        &mut self,
        resources: &HashMap<ResouceId, ResourceCount>
    ) {
        for (&resource_id, &resource_count) in resources {
            if let Some(allocated) = self.allocated.get_mut(&resource_id) {
                *allocated -= resource_count;

                if resource_count == 0 {
                    self.allocated.remove(&resource_id);
                }
            }

            if let Some(needed) = self.needed.get_mut(&resource_id) {
                *needed += resource_count;
            }
        }
    }

    pub fn can_finish(
        &self,
        available: &HashMap<ResouceId, ResourceCount>
    ) -> bool {
        self.needed.iter().all(|(&resource_id, &needed_count)| {
            available.get(&resource_id).unwrap_or(&0) >= &needed_count
        })
    }
}