use std::str::FromStr;

use axum::Json;
use serde::{Deserialize, Serialize};
use solana_sdk::system_instruction;
use solana_program::pubkey::Pubkey;
use base64;
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
    let from = Pubkey::from_str(&req.from).unwrap();
    let to = Pubkey::from_str(&req.to).unwrap();

    let ix = system_instruction::transfer(&from, &to, req.lamports);

    let accounts = ix.accounts.iter().map(|meta| meta.pubkey.to_string()).collect();
    let instruction_data = base64::encode(ix.data);

    Json(ApiResponse::success(SendSolResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    }))
}