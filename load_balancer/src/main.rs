use std::{collections::HashMap, convert::Infallible, net::SocketAddr, str::FromStr, sync::Arc};
mod utils;
use utils::constants;

use hyper::{
    body::HttpBody,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, StatusCode,
};
use load_balancer::{
    strategy::{strategy_manager, StrategyManager, StrategyNames},
    utils::init_tracing,
    FastestServerStrategy, LoadBalancer, RandomStrategy, RoundRobinStrategy,
};
use tokio::sync::RwLock;

async fn handle_default(
    req: Request<Body>,
    load_balancer: Arc<RwLock<LoadBalancer>>,
) -> Result<Response<Body>, hyper::Error> {
    load_balancer.write().await.forward_request(req).await.await
}

// visit /set_strategy/{strategy_name}
#[tracing::instrument(skip_all)]
async fn handle_set_strategy(
    req: Request<Body>,
    load_balancer: Arc<RwLock<LoadBalancer>>,
) -> Result<Response<Body>, hyper::Error> {
    let params: HashMap<String, String> = req
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .expect("Could not parse query parameters");
    match params.get(constants::STRATEGY) {
        Some(strategy) => {
            // TODO: Handle the error properly, instead of crashing the server
            let new_strategy = StrategyNames::from_str(strategy.as_str())
                .unwrap_or_else(|_e| panic!("Could not parse strategy"));
            let existing_strategy = load_balancer.read().await.get_strategy();

            load_balancer.write().await.set_strategy(new_strategy);
            let mut response_builder = Response::builder();

            // Optionally, set headers
            response_builder =
                response_builder.header(constants::CONTENT_TYPE, constants::TEXT_PLAIN);

            // Build the response with a 200 OK status and a body
            let response = response_builder
                .status(StatusCode::OK)
                .body(Body::from(format!(
                    "original strategy:{}, new strategy:{}",
                    existing_strategy, new_strategy
                )))
                .unwrap_or_else(|e| {
                    tracing::error!("Failed to build response: {}", e);
                    panic!("Failed to build response")
                });
            Ok(response)
        }
        // TODO: Handle the error properly, instead of crashing the server
        None => {
            tracing::error!("No strategy provided");
            panic!("No strategy provided");
        }
    }
}

async fn router(
    req: Request<Body>,
    load_balancer: Arc<RwLock<LoadBalancer>>,
) -> Result<Response<Body>, hyper::Error> {
    match req.uri().path() {
        "/set-strategy" => handle_set_strategy(req, load_balancer).await,
        _ => handle_default(req, load_balancer).await,
    }
}
#[tokio::main]
async fn main() {
    color_eyre::install().expect("Failed to install color_eyre");
    init_tracing().expect("Failed to initialize tracing");
    let worker_hosts: Vec<String> = if std::env::var("CONTAINER").is_ok() {
        // if this package is being run inside of a container
        vec![
            "http://worker-000:3000".to_string(),
            "http://worker-001:3000".to_string(),
            "http://worker-002:3000".to_string(),
        ]
    } else {
        // if this package is being run locally, but the workers are running in containerswj
        vec![
            "http://localhost:3000".to_string(),
            "http://localhost:3001".to_string(),
            "http://localhost:3002".to_string(),
        ]
    };

    // TODO: change strategy manager to take Arc<Vec<String>> instead of Vec<String>. I tried this
    // but couldn't implement it for the FastestServerStrategy strategy, becuase of the way that
    // I measure the fastest server - I have to consruct a WorkerDelay instance, and couldn't
    // figure out how to get the lifetimes working

    // TODO: allow the user to pass in the default strategy via a command line argument when
    // starting the application
    let strategy_manager = StrategyManager::new(worker_hosts, Option::from(StrategyNames::Random));
    let load_balancer = Arc::new(RwLock::new(
        LoadBalancer::new(strategy_manager)
            .await
            .expect("failed to create load balancer"),
    ));

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 1337));
    let make_svc = make_service_fn(move |_conn| {
        let load_balancer = load_balancer.clone();
        async { Ok::<_, Infallible>(service_fn(move |_req| router(_req, load_balancer.clone()))) }
    });

    let server = Server::bind(&addr).serve(make_svc);
    // let server = Server::bind(&addr).serve(make_service_fn(move |_conn| {
    //     let load_balancer = load_balancer.clone();
    //     async move { Ok::<_, Infallible>(service_fn(move |req| handle(req, load_balancer.clone()))) }
    // }));

    if let Err(e) = server.await {
        println!("error: {}", e);
    }
}
