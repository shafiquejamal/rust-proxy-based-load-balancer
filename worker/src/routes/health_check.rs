use std::time::Duration;

use axum::{http::StatusCode, response::IntoResponse};

use crate::utils::constants::HEALTH_CHECK_DELAY_MS;

pub async fn health_check() -> impl IntoResponse {
    std::thread::sleep(Duration::from_millis(*HEALTH_CHECK_DELAY_MS));
    (StatusCode::OK, format!("delay: {}", *HEALTH_CHECK_DELAY_MS))
}
