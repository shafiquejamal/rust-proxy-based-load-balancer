use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
};

use derive_more::Display;
use tracing::Level;

use crate::utils::MAX_ALLOWABLE_LATENCIES_STORED_PER_WORKER;

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
    latency_workers,
    connections_count
)]
pub struct PerformanceMetrics {
    // TODO: replace string with Arc<String> or Arc<Host> something
    pub latency_workers: LatencyWorkers,
    pub connections_count: BinaryHeap<Reverse<WorkerConnectionsCount>>,
}

impl PerformanceMetrics {
    pub fn new(worker_hosts: Vec<String>) -> Self {
        Self {
            latency_workers: LatencyWorkers::new(worker_hosts.clone()),
            connections_count: BinaryHeap::new(),
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn update_latency(&mut self, host: &String, new_latency: u128) {
        tracing::event!(Level::INFO, host, new_latency, "Updating latency");
        // Assumes that the best performing worker is still at the top of the heap
        let mut maybe_existing_latencies = self.latency_workers.worker_latencies.get_mut(host);
        let mut new_average_latency: u128 = 1;
        if let Some(existing_latencies) = maybe_existing_latencies {
            // for lat in existing_latencies.iter() {
            //     tracing::event!(Level::INFO, latency = lat);
            // }
            tracing::event!(
                Level::INFO,
                existing_latencies_length = existing_latencies.len(),
                new_latency,
                max_allowed_latencies_stored_per_worker = MAX_ALLOWABLE_LATENCIES_STORED_PER_WORKER
            );
            if existing_latencies.len() > MAX_ALLOWABLE_LATENCIES_STORED_PER_WORKER {
                loop {
                    existing_latencies.pop_back();
                    if existing_latencies.len() <= MAX_ALLOWABLE_LATENCIES_STORED_PER_WORKER {
                        break;
                    }
                }
            }
            existing_latencies.push_front(new_latency);
            // TODO: replace the cast with something safer
            new_average_latency =
                existing_latencies.iter().sum::<u128>() / (existing_latencies.len() as u128);
            existing_latencies.push_front(new_average_latency);
            tracing::event!(Level::INFO, new_latency, new_average_latency,);
        } else {
            let mut queue = VecDeque::new();
            queue.push_front(new_average_latency);
            tracing::event!(
                Level::INFO,
                new_queue_length = queue.len(),
                new_latency,
                new_average_latency,
            );
            self.latency_workers
                .worker_latencies
                .insert(host.clone(), queue);
        }
        let maybe_worker = self.latency_workers.workers.peek_mut();
        tracing::event!(Level::INFO, new_average_latency);
        if let Some(mut worker) = maybe_worker {
            worker.0.delay_ms = new_average_latency;
        }
    }
}

#[derive(Debug)]
pub struct LatencyWorkers {
    pub workers: BinaryHeap<Reverse<WorkerDelay>>,
    pub worker_latencies: HashMap<String, VecDeque<u128>>,
}

impl LatencyWorkers {
    pub fn new(mut worker_hosts: Vec<String>) -> Self {
        let heap = BinaryHeap::from_iter(
            worker_hosts
                .drain(..)
                .map(|host| Reverse(WorkerDelay { host, delay_ms: 0 })),
        );
        let mut map = HashMap::new();
        for host in worker_hosts.iter() {
            map.insert(host.clone(), VecDeque::new());
        }
        Self {
            workers: heap,
            worker_latencies: map,
        }
    }
}
