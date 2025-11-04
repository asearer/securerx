use axum::{Json, extract::Path, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use securerx_core::transaction::Transaction;
use securerx_core::crypto::{generate_keypair, sign_message};
use std::sync::{Arc, Mutex};
use securerx_core::blockchain::Blockchain;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub blockchain: Arc<Mutex<Blockchain>>,
}

/// Request payload to issue a prescription
#[derive(Deserialize)]
pub struct PrescriptionRequest {
    pub doctor_id: String,
    pub patient_id: String,
    pub drug: String,
}

/// Response payload for submission
#[derive(Serialize)]
pub struct PrescriptionResponse {
    pub status: String,
    pub block_index: u64,
}

/// Endpoint: Health check
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({"status": "ok"})))
}

/// Endpoint: Submit a prescription
pub async fn submit_prescription(
    state: axum::extract::Extension<AppState>,
    Json(payload): Json<PrescriptionRequest>,
) -> impl IntoResponse {
    let keypair = generate_keypair(); // Simulated signing per doctor
    let sig = sign_message(&keypair, payload.drug.as_bytes());

    let tx = Transaction {
        doctor_id: payload.doctor_id,
        patient_id: payload.patient_id,
        drug: payload.drug,
        signature: sig.to_bytes().to_vec(),
        pubkey: keypair.verifying_key().to_bytes().to_vec(),
    };

    let mut blockchain = state.blockchain.lock().unwrap();
    let block = blockchain.add_block(vec![tx]);

    (StatusCode::CREATED, Json(PrescriptionResponse {
        status: "success".to_string(),
        block_index: block.index,
    }))
}

/// Endpoint: Query blockchain
pub async fn get_chain(
    state: axum::extract::Extension<AppState>,
) -> impl IntoResponse {
    let blockchain = state.blockchain.lock().unwrap();
    Json(blockchain.chain.clone())
}

/// Endpoint: Get a specific block by index
pub async fn get_block(
    state: axum::extract::Extension<AppState>,
    Path(index): Path<usize>,
) -> impl IntoResponse {
    let blockchain = state.blockchain.lock().unwrap();
    if index < blockchain.chain.len() {
        (StatusCode::OK, Json(Some(blockchain.chain[index].clone())))
    } else {
        (StatusCode::NOT_FOUND, Json(None::<securerx_core::block::Block>))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::{get, post},
        Router,
        Extension,
    };
    use tower::ServiceExt;
    use std::sync::{Arc, Mutex};

    fn create_app() -> Router {
        let blockchain = Arc::new(Mutex::new(Blockchain::new()));
        let app_state = AppState { blockchain };
        
        Router::new()
            .route("/health", get(health))
            .route("/prescription", post(submit_prescription))
            .route("/blocks", get(get_chain))
            .route("/blocks/:index", get(get_block))
            .layer(Extension(app_state))
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = create_app();
        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_submit_prescription() {
        let app = create_app();
        let payload = serde_json::json!({
            "doctor_id": "doctor1",
            "patient_id": "patient1",
            "drug": "Aspirin"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/prescription")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_get_chain() {
        let app = create_app();
        let response = app
            .oneshot(Request::builder().uri("/blocks").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_block_valid_index() {
        let app = create_app();
        // First add a block
        let payload = serde_json::json!({
            "doctor_id": "doctor1",
            "patient_id": "patient1",
            "drug": "Aspirin"
        });
        
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/prescription")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        let response = app
            .oneshot(Request::builder().uri("/blocks/0").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_block_invalid_index() {
        let app = create_app();
        let response = app
            .oneshot(Request::builder().uri("/blocks/999").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
