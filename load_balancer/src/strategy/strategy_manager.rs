use core::str;
use std::collections::HashMap;

use crate::{
    strategy::{fastest_server, Strategy, StrategyNames},
    FastestServerStrategy, RandomStrategy, RoundRobinStrategy,
};

pub struct StrategyManager {
    current_strategy: StrategyNames,
    all_strategies: HashMap<StrategyNames, Box<dyn Strategy>>,
}

impl StrategyManager {
    pub fn new(worker_hosts: Vec<String>, default_strategy: Option<StrategyNames>) -> Self {
        // TODO:: add a check that the current strategy is in the map of all strategies
        let random_strategy = RandomStrategy::new(worker_hosts.clone());
        let fastest_server_strategy = FastestServerStrategy::new(worker_hosts.clone());
        let round_robin_strategy = RoundRobinStrategy::new(worker_hosts);
        // TODO: How would I implement FEWEST CONNECTIONS? Can the LB keep track of the
        // number of connections?
        let mut all_strategies: HashMap<StrategyNames, Box<dyn Strategy>> = HashMap::new();

        all_strategies.insert(StrategyNames::Random, Box::new(random_strategy));
        all_strategies.insert(
            StrategyNames::FastestServer,
            Box::new(fastest_server_strategy),
        );
        all_strategies.insert(StrategyNames::RoundRobin, Box::new(round_robin_strategy));
        Self {
            current_strategy: default_strategy.unwrap_or(StrategyNames::Random),
            all_strategies,
        }
    }

    // TODO: anyway to flatmap? Call the .await inside the monad? I don't like the double map and
    // the unwrap call
    pub async fn get_worker(&mut self) -> Option<String> {
        let strategy = self.all_strategies.get_mut(&self.current_strategy);
        let worker = strategy
            .map(async |b| b.get_worker().await.map(|w| w.clone()))
            .unwrap()
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
