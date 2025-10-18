use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Binary,
};

use derive_more::Display;

#[derive(Debug, Clone, Display)]
pub enum PerformanceMetricsNames {
    RequestLatency,
    RequestCount,
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
// TODO: This is not used, but I should use it in the newtype pattern
pub struct Latency {
    pub ms: u128,
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
// TODO: create constructor
pub struct ConnectionsCount {
    pub count: u64,
}

#[derive(Eq, PartialEq, Display, Debug)]
#[display("host:{host}, delay:{delay_ms}")]
pub struct WorkerDelay {
    pub host: String,
    pub delay_ms: u128,
}

impl Ord for WorkerDelay {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.delay_ms.cmp(&other.delay_ms)
    }
}

impl PartialOrd for WorkerDelay {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.delay_ms.cmp(&other.delay_ms))
    }
}

#[derive(Eq, PartialEq, Display, Debug)]
#[display("host:{host}, connections_count:{connections_count}")]
pub struct WorkerConnectionsCount {
    pub host: String,
    pub connections_count: ConnectionsCount,
}

impl Ord for WorkerConnectionsCount {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.connections_count
            .count
            .cmp(&other.connections_count.count)
    }
}

impl PartialOrd for WorkerConnectionsCount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.connections_count
                .count
                .cmp(&other.connections_count.count),
        )
    }
}

#[derive(Debug, Display)]
#[display(
    "latency_ms:{:?}, connections_count:{:?}",
    latency_ms,
    connections_count
)]
pub struct PerformanceMetrics {
    // TODO: replace string with Arc<String> or Arc<Host> something
    pub latency_ms: BinaryHeap<Reverse<WorkerDelay>>,
    pub connections_count: BinaryHeap<Reverse<WorkerConnectionsCount>>,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            latency_ms: BinaryHeap::new(),
            connections_count: BinaryHeap::new(),
        }
    }
}
