use prometheus::{IntCounter, IntGauge, register_int_counter, register_int_gauge};
use lazy_static::lazy_static;

/// Prometheus metrics for node observability
lazy_static! {
    pub static ref BLOCKS_PROCESSED: IntCounter = register_int_counter!(
        "blocks_processed_total",
        "Total number of blocks processed by this node"
    ).unwrap();

    pub static ref TRANSACTIONS_PROCESSED: IntCounter = register_int_counter!(
        "transactions_processed_total",
        "Total number of transactions processed by this node"
    ).unwrap();

    pub static ref CHAIN_HEIGHT: IntGauge = register_int_gauge!(
        "chain_height",
        "Current blockchain height"
    ).unwrap();
}
