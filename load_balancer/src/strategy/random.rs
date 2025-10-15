use crate::strategy::Strategy;
use rand::prelude::*;

#[derive(Debug)]
pub struct RandomStrategy {
    worker_hosts: Vec<String>,
}

#[async_trait::async_trait]
impl Strategy for RandomStrategy {
    #[tracing::instrument(skip_all)]
    async fn get_worker(&mut self) -> Option<String> {
        let mut rng = rand::rng();
        return self
            .worker_hosts
            .choose(&mut rng)
            .map(|worker| worker.clone());
    }
}

impl RandomStrategy {
    #[tracing::instrument(name = "Create RandomStrategy with worker_hosts")]
    pub fn new(worker_hosts: Vec<String>) -> Self {
        Self { worker_hosts }
    }
}
