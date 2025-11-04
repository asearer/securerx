use axum::{Router, routing::get, response::IntoResponse};
use std::net::SocketAddr;
use tokio::task;
use securerx_node::{config::NodeConfig, node::Node};
use prometheus::{Encoder, TextEncoder};

#[tokio::main]
async fn main() {
    // Load configuration
    let config = NodeConfig::from_env();
    let node = Node::new(config);

    let node_clone = node.clone();
    // Start gossip loop
    task::spawn(async move {
        node_clone.gossip_loop().await;
    });

    // Metrics endpoint
    let metrics_route = Router::new().route("/metrics", get(metrics_handler));
    let addr: SocketAddr = node.config.api_addr.parse().unwrap();
    println!("Node {} listening on {}", node.config.node_id, addr);
    axum::Server::bind(&addr).serve(metrics_route.into_make_service()).await.unwrap();
}

/// Prometheus metrics handler
async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
