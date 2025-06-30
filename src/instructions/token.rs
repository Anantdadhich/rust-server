use std::str::FromStr;

use axum::Json;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use spl_token::instruction::transfer;
use spl_associated_token_account::get_associated_token_address;
use base64::{engine::general_purpose, Engine as _};

#[derive(Deserialize)]
pub struct SendTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct AccountInfo {
    pub pubkey: String,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

#[derive(Serialize)]
pub struct SendTokenData {
    pub program_id: String,
    pub accounts: Vec<AccountInfo>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct SendTokenResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SendTokenData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub async fn send_token(Json(req): Json<SendTokenRequest>) -> Json<SendTokenResponse> {
    if req.amount == 0 {
        return Json(SendTokenResponse {
            success: false,
            data: None,
            error: Some("Amount must be greater than 0".to_string()),
        });
    }

    let destination = match Pubkey::from_str(&req.destination) {
        Ok(p) => p,
        Err(_) => return error_response("Invalid destination public key"),
    };

    let mint = match Pubkey::from_str(&req.mint) {
        Ok(p) => p,
        Err(_) => return error_response("Invalid mint public key"),
    };

    let owner = match Pubkey::from_str(&req.owner) {
        Ok(p) => p,
        Err(_) => return error_response("Invalid owner public key"),
    };


    let source_ata = get_associated_token_address(&owner, &mint);
    let destination_ata = get_associated_token_address(&destination, &mint);

    let ix = match transfer(
        &spl_token::id(),
        &source_ata,
        &destination_ata,
        &owner,
        &[&owner],
        req.amount,
    ) {
        Ok(ix) => ix,
        Err(e) => return error_response(&format!("Failed to create transfer instruction: {}", e)),
    };

    let accounts = ix
        .accounts
        .iter()
        .map(|meta| AccountInfo {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
        })
        .collect();

    let instruction_data = general_purpose::STANDARD.encode(&ix.data);

    Json(SendTokenResponse {
        success: true,
        data: Some(SendTokenData {
            program_id: ix.program_id.to_string(),
            accounts,
            instruction_data,
        }),
        error: None,
    })
}


fn error_response(msg: &str) -> Json<SendTokenResponse> {
    Json(SendTokenResponse {
        success: false,
        data: None,
        error: Some(msg.to_string()),
    })
}
