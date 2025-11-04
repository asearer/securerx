use axum::{
    routing::{get, post},
    Router, Extension,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use securerx_core::blockchain::Blockchain;
use securerx_api::handlers::{health, submit_prescription, get_chain, get_block, AppState};

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let app_state = AppState { blockchain };

    let app = Router::new()
        .route("/health", get(health))
        .route("/prescription", post(submit_prescription))
        .route("/blocks", get(get_chain))
        .route("/blocks/:index", get(get_block))
        .layer(Extension(app_state));

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    println!("API server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
