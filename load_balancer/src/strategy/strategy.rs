use derive_more::{Display, FromStr};

#[async_trait::async_trait]
// TODO: is adding Send + Sync OK here?
pub trait Strategy: Send + Sync {
    async fn get_worker(&mut self) -> Option<String>;
}

#[derive(Debug, Display, Eq, PartialEq, Hash, FromStr, Clone, Copy)]
pub enum StrategyNames {
    RoundRobin,
    FastestServer,
    Random,
    FewestConnections,
}
