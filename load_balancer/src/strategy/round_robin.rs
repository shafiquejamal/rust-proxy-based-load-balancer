use crate::strategy::Strategy;

pub struct RoundRobinStrategy {
    worker_hosts: Vec<String>,
    current_worker: usize,
}

#[async_trait::async_trait]
impl Strategy for RoundRobinStrategy {
    async fn get_worker(&mut self) -> Option<String> {
        if self.worker_hosts.is_empty() {
            return None;
        }
        let worker = self.worker_hosts.get(self.current_worker).unwrap().clone();
        self.current_worker = (self.current_worker + 1) % self.worker_hosts.len();
        Some(worker)
    }
}

impl RoundRobinStrategy {
    pub fn new(worker_hosts: Vec<String>) -> Self {
        Self {
            worker_hosts,
            current_worker: 0,
        }
    }
}
