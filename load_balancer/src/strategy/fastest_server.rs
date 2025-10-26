use std::sync::Arc;
use tokio::sync::RwLock;

use crate::performance::PerformanceMetrics;
use crate::strategy::Strategy;

#[derive(Debug)]
pub struct FastestServerStrategy {
    performance_metrics: Arc<RwLock<PerformanceMetrics>>,
}

#[async_trait::async_trait]
pub trait MyHTTPClient {
    async fn request();
}

#[async_trait::async_trait]
impl Strategy for FastestServerStrategy {
    #[tracing::instrument(skip_all)]
    async fn get_worker(&mut self) -> Option<String> {
        let mut workers = self.performance_metrics.write().await;
        let fastest_worker = workers.latency_workers.workers.peek_mut();
        return fastest_worker.map(|w| w.0.host.clone());
    }
}

impl FastestServerStrategy {
    #[tracing::instrument(name = "Create FastestServerStrategy with worker_hosts")]
    pub fn new(performance_metrics: Arc<RwLock<PerformanceMetrics>>) -> Self {
        Self {
            performance_metrics,
        }
    }
}
