use hyper::{client::ResponseFuture, Body, Client, Request, Uri};
use std::str::FromStr;
use tracing::Level;

pub mod strategy;
pub mod utils;

pub use strategy::{FastestServerStrategy, RandomStrategy, RoundRobinStrategy};

use crate::strategy::{StrategyManager, StrategyNames};

pub struct LoadBalancer {
    client: Client<hyper::client::HttpConnector>,
    strategy_manager: StrategyManager,
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
    // TODO: Is this the right approach, to have a strategy manager?
    pub async fn new(mut strategy_manager: StrategyManager) -> Result<Self, String> {
        if strategy_manager.get_worker().await.is_none() {
            return Err("No worker hosts provided".into());
        }

        Ok(LoadBalancer {
            client: Client::new(),
            strategy_manager,
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

        self.client.request(new_req)
    }

    async fn get_worker(&mut self) -> String {
        self.strategy_manager.get_worker().await.unwrap()
    }
}
