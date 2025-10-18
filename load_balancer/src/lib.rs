use hyper::{client::ResponseFuture, Body, Client, Request, Uri};
use std::{cmp::Reverse, str::FromStr, sync::Arc};
use tokio::{sync::RwLock, time::Instant};
use tracing::Level;

pub mod performance;
pub mod strategy;
pub mod utils;

use crate::{
    performance::{
        metrics::PerformanceMetrics, ConnectionsCount, WorkerConnectionsCount, WorkerDelay,
    },
    strategy::{StrategyManager, StrategyNames},
};
pub use strategy::{FastestServerStrategy, RandomStrategy, RoundRobinStrategy};

pub struct LoadBalancer {
    client: Client<hyper::client::HttpConnector>,
    strategy_manager: StrategyManager,
    perforamance_metrics: Arc<RwLock<PerformanceMetrics>>,
}

impl LoadBalancer {
    pub fn get_strategy(&self) -> StrategyNames {
        self.strategy_manager.get_strategy()
    }
    #[tracing::instrument(skip_all)]
    pub fn set_strategy(&mut self, new_strategy: StrategyNames) {
        let existing_strategy = self.strategy_manager.get_strategy();
        self.strategy_manager.set_strategy(new_strategy);
        tracing::event!(
            Level::INFO,
            existing_strategy = format!("{}", existing_strategy),
            new_strategy = format!("{}", new_strategy)
        );
    }

    pub async fn new(
        mut strategy_manager: StrategyManager,
        perforamance_metrics: Arc<RwLock<PerformanceMetrics>>,
    ) -> Result<Self, String> {
        if strategy_manager.get_worker().await.is_none() {
            return Err("No worker hosts provided".into());
        }

        Ok(LoadBalancer {
            client: Client::new(),
            strategy_manager,
            perforamance_metrics,
        })
    }

    #[tracing::instrument(skip_all)]
    pub async fn forward_request(&mut self, req: Request<Body>) -> ResponseFuture {
        let mut worker_uri = self.get_worker().await;
        tracing::event!(Level::INFO, worker_uri);

        // Extract the path and query from the original request
        if let Some(path_and_query) = req.uri().path_and_query() {
            worker_uri.push_str(path_and_query.as_str());
        }

        // Create a new URI from the worker URI
        let new_uri = Uri::from_str(&worker_uri).unwrap();

        // Extract the headers from the original request
        let headers = req.headers().clone();

        // Clone the original request's headers and method
        let mut new_req = Request::builder()
            .method(req.method())
            .uri(new_uri)
            .body(req.into_body())
            .expect("request builder");

        // Copy headers from the original request
        for (key, value) in headers.iter() {
            new_req.headers_mut().insert(key, value.clone());
        }

        let start = Instant::now();
        tracing::event!(Level::INFO, "Sending request",);
        let response = self.client.request(new_req);
        tracing::event!(Level::INFO, "Response received",);
        let duration = start.elapsed().as_millis();
        // TODO: create a new/parse function to hide the use of Reverse
        let worker_delay = WorkerDelay {
            host: worker_uri.clone(),
            delay_ms: duration,
        };
        // TODO: The following line, when uncommentded, prevents a response from being
        // returned
        // self.perforamance_metrics
        //     .write()
        //     .await
        //     .latency_ms
        //     .push(Reverse(worker_delay));
        response
    }

    async fn get_worker(&mut self) -> String {
        self.strategy_manager.get_worker().await.unwrap()
    }
}
