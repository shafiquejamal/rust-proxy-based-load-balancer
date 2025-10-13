use hyper::{client::ResponseFuture, Body, Client, Request, Uri};
use std::str::FromStr;

mod strategy;
pub mod utils;

use strategy::Strategy;
pub use strategy::{FastestServerStrategy, RoundRobinStrategy};

pub struct LoadBalancer {
    client: Client<hyper::client::HttpConnector>,
    strategy: Box<dyn Strategy>,
}

impl LoadBalancer {
    pub async fn new(mut strategy: Box<dyn Strategy>) -> Result<Self, String> {
        if strategy.get_worker().await.is_none() {
            return Err("No worker hosts provided".into());
        }

        Ok(LoadBalancer {
            client: Client::new(),
            strategy,
        })
    }

    pub async fn forward_request(&mut self, req: Request<Body>) -> ResponseFuture {
        let mut worker_uri = self.get_worker().await;

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
        self.strategy.get_worker().await.unwrap()
    }
}
