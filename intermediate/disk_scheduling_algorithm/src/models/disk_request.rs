#[derive(Debug, Clone)]
pub struct DiskRequest {
    pub position: u32,
}

impl DiskRequest {
    pub fn new(position: u32) -> Self {
        DiskRequest { 
            position 
        }
    }
}