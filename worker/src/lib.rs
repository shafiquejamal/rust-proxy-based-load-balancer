use std::error::Error;

use axum::routing::get;
use axum::{Router, serve::Serve};
use tower_http::services::ServeDir;

pub mod routes;
pub mod utils;

pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        // DONE

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/health-check", get(routes::health_check));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        // DONE
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
