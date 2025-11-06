use core::str;
use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::{
    performance::PerformanceMetrics,
    strategy::{fastest_server, FewestConnectionsStrategy, Strategy, StrategyNames},
    FastestServerStrategy, RandomStrategy, RoundRobinStrategy,
};

pub struct StrategyManager {
    current_strategy: StrategyNames,
    all_strategies: HashMap<StrategyNames, Box<dyn Strategy>>,
}

impl StrategyManager {
    pub fn new(
        worker_hosts: Vec<String>,
        default_strategy: Option<StrategyNames>,
        performance_metrics: Arc<RwLock<PerformanceMetrics>>,
    ) -> Self {
        let random_strategy = RandomStrategy::new(worker_hosts.clone());
        let fastest_server_strategy = FastestServerStrategy::new(performance_metrics.clone());
        let round_robin_strategy = RoundRobinStrategy::new(worker_hosts);
        let fewest_connections = FewestConnectionsStrategy::new(performance_metrics);
        let mut all_strategies: HashMap<StrategyNames, Box<dyn Strategy>> = HashMap::new();

        all_strategies.insert(StrategyNames::Random, Box::new(random_strategy));
        all_strategies.insert(
            StrategyNames::FastestServer,
            Box::new(fastest_server_strategy),
        );

        all_strategies.insert(StrategyNames::RoundRobin, Box::new(round_robin_strategy));
        all_strategies.insert(
            StrategyNames::FewestConnections,
            Box::new(fewest_connections),
        );
        Self {
            current_strategy: default_strategy.unwrap_or(StrategyNames::FastestServer),
            all_strategies,
        }
    }

    // TODO: anyway to flatmap? Call the .await inside the monad? I don't like the double map and
    // the unwrap call
    pub async fn get_worker(&mut self) -> Option<String> {
        let strategy = self.all_strategies.get_mut(&self.current_strategy);
        let worker = strategy
            .map(async |b| b.get_worker().await.map(|w| w.clone()))
            .expect("Could not get the worker from the strategy")
            .await;
        return worker;
    }

    pub fn get_strategy(&self) -> StrategyNames {
        self.current_strategy
    }

    pub fn set_strategy(&mut self, strategy: StrategyNames) {
        self.current_strategy = strategy;
    }
}
