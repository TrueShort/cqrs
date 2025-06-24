use crate::{Aggregate, AggregateContext};
use std::fmt::Debug;

/// Holds context for the pure event store implementation PostgresStore.
/// This is only used internally within the `EventStore`.
#[derive(Debug)]
pub struct EventStoreAggregateContext<A: Aggregate + Debug> {
    /// The aggregate ID of the aggregate instance that has been loaded.
    pub aggregate_id: String,
    /// The current state of the aggregate instance.
    pub aggregate: A,
    /// The last committed event sequence number for this aggregate instance.
    pub current_sequence: usize,
    /// The last committed snapshot version for this aggregate instance.
    pub current_snapshot: Option<usize>,
}

impl<A: Aggregate + Debug> EventStoreAggregateContext<A> {
    pub(crate) fn context_for(aggregate_id: &str, _is_event_source: bool) -> Self {
        Self {
            aggregate_id: aggregate_id.to_string(),
            aggregate: A::default(),
            current_sequence: 0,
            current_snapshot: None,
        }
    }
}

impl<A: Aggregate + Debug> AggregateContext<A> for EventStoreAggregateContext<A> {
    fn aggregate(&self) -> &A {
        &self.aggregate
    }
}
