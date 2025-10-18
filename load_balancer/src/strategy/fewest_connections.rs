use std::sync::Arc;
use tokio::sync::RwLock;

use crate::performance::PerformanceMetrics;
use crate::strategy::Strategy;

#[derive(Debug)]
pub struct FewestConnectionsStrategy {
    performance_metrics: Arc<RwLock<PerformanceMetrics>>,
}

#[async_trait::async_trait]
pub trait MyHTTPClient {
    async fn request();
}

#[async_trait::async_trait]
impl Strategy for FewestConnectionsStrategy {
    #[tracing::instrument(skip_all)]
    async fn get_worker(&mut self) -> Option<String> {
        let workers = self.performance_metrics.read().await;
        let fewest_connection_worker = workers
            .connections_count
            .peek()
            .map(|worker| worker.0.host.clone());
        fewest_connection_worker
    }
}

impl FewestConnectionsStrategy {
    #[tracing::instrument(name = "Create FastestServerStrategy with worker_hosts")]
    pub fn new(performance_metrics: Arc<RwLock<PerformanceMetrics>>) -> Self {
        Self {
            performance_metrics,
        }
    }
}

#[cfg(test)]
mod tests {
    use mock_instant::global::{Instant, SystemTime};
    use std::time::Duration;

    // TODO: I need to figure out how to mock Instant, hyper Client
}
