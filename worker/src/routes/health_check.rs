use rand::prelude::*;
use std::time::Duration;

use axum::{http::StatusCode, response::IntoResponse};

use crate::utils::constants::WORKER_NUMBER;

pub async fn health_check() -> impl IntoResponse {
    let mut rng = rand::rng();
    let delay_ms: u64 = rng.random_range(0..=100);
    // std::thread::sleep(Duration::from_millis(delay_ms));
    (
        StatusCode::OK,
        format!(
            "worker: {}, (not implemented) delay: {}",
            *WORKER_NUMBER, delay_ms
        ),
    )
}
