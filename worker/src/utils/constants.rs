use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

lazy_static! {
    pub static ref HEALTH_CHECK_DELAY_MS: u64 = set_health_check_delay_ms();
    pub static ref WORKER_NUMBER: &'static str = set_worker_number();
}

fn set_worker_number() -> &'static str {
    dotenv().ok();
    std_env::var(env::WORKER_NUMBER_VAR)
        .unwrap_or_else(|_| "3000".to_string())
        .leak()
}

fn set_health_check_delay_ms() -> u64 {
    dotenv().ok();
    std_env::var(env::HEALTH_CHECK_DELAY_MS_ENV_VAR)
        .expect("HEALTH_CHECK_DELAY_MS_ENV_VAR must be set")
        .parse()
        .expect("Error - could not parse value for env var HEALTH_CHECK_DELAY_MS_ENV_VAR")
}

pub mod env {
    pub const HEALTH_CHECK_DELAY_MS_ENV_VAR: &str = "HEALTH_CHECK_DELAY_MS";
    pub const WORKER_NUMBER_VAR: &str = "WORKER_NUMBER";
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}
