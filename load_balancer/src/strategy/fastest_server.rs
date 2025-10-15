use derive_more::Display;
use std::time::Instant;
use std::{cmp::Reverse, collections::BinaryHeap};

use hyper::{Body, Client, Request};
use tracing::Level;

use crate::strategy::Strategy;

#[derive(Eq, PartialEq, Display, Debug)]
#[display("host:{host}, delay:{delay_ms}")]
pub struct WorkerDelay {
    pub host: String,
    pub delay_ms: u128,
}

impl Ord for WorkerDelay {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.delay_ms.cmp(&other.delay_ms)
    }
}

impl PartialOrd for WorkerDelay {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.delay_ms.cmp(&other.delay_ms))
    }
}

#[derive(Debug)]
pub struct FastestServerStrategy {
    worker_hosts: BinaryHeap<Reverse<WorkerDelay>>,
    client: Client<hyper::client::HttpConnector>, // Box<dyn MyHTTPClient>,
}

#[async_trait::async_trait]
pub trait MyHTTPClient {
    async fn request();
}

#[async_trait::async_trait]
impl Strategy for FastestServerStrategy {
    #[tracing::instrument(skip_all)]
    async fn get_worker(&mut self) -> Option<String> {
        let fastest_worker = self.worker_hosts.peek_mut();
        match fastest_worker {
            None => return None,
            Some(mut worker) => {
                let start = Instant::now();
                let health_check_endpoint = format!("{}/health-check", &worker.0.host);
                let _ = &self
                    .client
                    .request(
                        Request::builder()
                            .method("GET")
                            .uri(&health_check_endpoint)
                            .body(Body::empty())
                            .unwrap(),
                    )
                    .await
                    .ok()?;
                let duration = start.elapsed().as_millis();
                tracing::event!(
                    Level::INFO,
                    previous_delay = format!("previous_delay:{}", worker.0.delay_ms),
                    duration = format!("actual duration:{}", duration),
                    health_check_endpoint
                );
                (*worker).0.delay_ms = duration;
                return Some(worker.0.host.clone());
            }
        };
    }
}

impl FastestServerStrategy {
    #[tracing::instrument(name = "Create FastestServerStrategy with worker_hosts")]
    pub fn new(worker_hosts: Vec<String>) -> Self {
        Self::new_with_client(worker_hosts, None)
    }
    pub fn new_with_client(
        worker_hosts: Vec<String>,
        maybe_client: Option<Client<hyper::client::HttpConnector>>,
    ) -> Self {
        let mut heap = BinaryHeap::new();
        worker_hosts.into_iter().for_each(|worker_host| {
            let worker_host = WorkerDelay {
                host: worker_host,
                delay_ms: 0,
            };
            heap.push(Reverse(worker_host))
        });
        tracing::event!(Level::INFO, "Workers added to the heap");
        let client = maybe_client.unwrap_or_else(|| Client::new());
        Self {
            worker_hosts: heap,
            client,
        }
    }
}

#[cfg(test)]
mod tests {
    use mock_instant::global::{Instant, SystemTime};
    use std::time::Duration;

    // TODO: I need to figure out how to mock Instant, hyper Client
}
