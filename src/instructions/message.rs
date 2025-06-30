use axum::Json;
use serde::{Deserialize, Serialize};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use bs58;
use base64::{Engine as _, engine::general_purpose};
use crate::model::response::ApiResponse;

#[derive(Deserialize)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

pub async fn sign_message(Json(req): Json<SignMessageRequest>) -> Json<ApiResponse<SignMessageResponse>> {
    let secret_bytes = match bs58::decode(&req.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Json(ApiResponse::error("Invalid secret key encoding")),
    };
    
    if secret_bytes.len() != 32 {
        return Json(ApiResponse::error("Invalid secret key length"));
    }
    
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&secret_bytes);
    
    let signing_key = SigningKey::from_bytes(&key_bytes);
    let verifying_key = signing_key.verifying_key();
    
    let signature = signing_key.sign(req.message.as_bytes());
    let signature_b64 = general_purpose::STANDARD.encode(signature.to_bytes());
    let public_key_b58 = bs58::encode(verifying_key.to_bytes()).into_string();

    Json(ApiResponse::success(SignMessageResponse {
        signature: signature_b64,
        public_key: public_key_b58,
        message: req.message,
    }))
}

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct VerifyMessageResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

pub async fn verify_message(Json(req): Json<VerifyMessageRequest>) -> Json<ApiResponse<VerifyMessageResponse>> {
    let signature_bytes = match general_purpose::STANDARD.decode(&req.signature) {
        Ok(bytes) => bytes,
        Err(_) => return Json(ApiResponse::error("Invalid signature encoding")),
    };
    
    let pubkey_bytes = match bs58::decode(&req.pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Json(ApiResponse::error("Invalid public key encoding")),
    };
    
    if pubkey_bytes.len() != 32 {
        return Json(ApiResponse::error("Invalid public key length"));
    }
    
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&pubkey_bytes);
    
    let verifying_key = match VerifyingKey::from_bytes(&key_bytes) {
        Ok(pk) => pk,
        Err(_) => return Json(ApiResponse::error("Invalid public key")),
    };
    
    let signature = match Signature::try_from(&signature_bytes[..]) {
        Ok(sig) => sig,
        Err(_) => return Json(ApiResponse::error("Invalid signature")),
    };
    
    let valid = verifying_key.verify(req.message.as_bytes(), &signature).is_ok();

    Json(ApiResponse::success(VerifyMessageResponse {
        valid,
        message: req.message,
        pubkey: req.pubkey,
    }))
}