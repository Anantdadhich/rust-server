use std::str::FromStr;
use axum::Json;
use serde::{Deserialize, Serialize};
use solana_sdk::system_instruction;
use solana_program::pubkey::Pubkey;
use base64::{engine::general_purpose, Engine as _};
use crate::model::response::ApiResponse;

#[derive(Deserialize)]
pub struct SendSolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Serialize)]
pub struct SendSolResponse {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

pub async fn transfer_sol(Json(req): Json<SendSolRequest>) -> Json<ApiResponse<SendSolResponse>> {
    
    let from = match Pubkey::from_str(&req.from) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(ApiResponse::error("Invalid sender public key"));
        }
    };

    let to = match Pubkey::from_str(&req.to) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(ApiResponse::error("Invalid recipient public key"));
        }
    };

    
    let ix = system_instruction::transfer(&from, &to, req.lamports);

    
    let accounts: Vec<String> = ix.accounts.iter().map(|meta| meta.pubkey.to_string()).collect();
    let instruction_data = general_purpose::STANDARD.encode(ix.data);

    Json(ApiResponse::success(SendSolResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    }))
}
