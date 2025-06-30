use axum::Json;
use serde::Serialize;
use solana_sdk::{signature::Keypair, signer::Signer};
use bs58;
use crate::model::response::ApiResponse;

#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

pub async fn generate_keypair() -> Json<ApiResponse<KeypairResponse>> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();
    Json(ApiResponse::success(KeypairResponse { pubkey, secret }))
}