use crate::models::disk_request::DiskRequest;

mod models;
mod scheduler;
mod utils;

fn main() {
    let requests = vec![
        DiskRequest::new(1, 98),
        DiskRequest::new(2, 183),
        DiskRequest::new(3, 37),
        DiskRequest::new(4, 122),
        DiskRequest::new(5, 14),
    ];
}
