use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use load_balancer::{utils::init_tracing, FastestServerStrategy, LoadBalancer, RoundRobinStrategy};
use tokio::sync::RwLock;

async fn handle(
    req: Request<Body>,
    load_balancer: Arc<RwLock<LoadBalancer>>,
) -> Result<Response<Body>, hyper::Error> {
    load_balancer.write().await.forward_request(req).await.await
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

    // let round_robin_strategy = Box::new(RoundRobinStrategy::new(worker_hosts));
    let fasted_connection_strategy = Box::new(FastestServerStrategy::new(worker_hosts));
    let load_balancer = Arc::new(RwLock::new(
        LoadBalancer::new(fasted_connection_strategy)
            .await
            .expect("failed to create load balancer"),
    ));

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 1337));

    let server = Server::bind(&addr).serve(make_service_fn(move |_conn| {
        let load_balancer = load_balancer.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| handle(req, load_balancer.clone()))) }
    }));

    if let Err(e) = server.await {
        println!("error: {}", e);
    }
}
