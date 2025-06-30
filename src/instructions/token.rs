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
pub struct SendTokenResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SendTokenData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct SendTokenData {
    pub program_id: String,
    pub accounts: Vec<AccountInfo>,
    pub instruction_data: String,
}

pub async fn send_token(Json(req): Json<SendTokenRequest>) -> Json<SendTokenResponse> {
    // Validate amount
    if req.amount == 0 {
        return Json(SendTokenResponse {
            success: false,
            data: None,
            error: Some("Amount must be greater than 0".to_string()),
        });
    }

    // Parse and validate destination address
    let destination = match Pubkey::from_str(&req.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Json(SendTokenResponse {
                success: false,
                data: None,
                error: Some("Invalid destination public key".to_string()),
            });
        }
    };

    // Parse and validate mint address
    let mint = match Pubkey::from_str(&req.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Json(SendTokenResponse {
                success: false,
                data: None,
                error: Some("Invalid mint public key".to_string()),
            });
        }
    };

    // Parse and validate owner address
    let owner = match Pubkey::from_str(&req.owner) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Json(SendTokenResponse {
                success: false,
                data: None,
                error: Some("Invalid owner public key".to_string()),
            });
        }
    };

    // Get associated token accounts
    let source_ata = get_associated_token_address(&owner, &mint);
    let destination_ata = get_associated_token_address(&destination, &mint);

    // Create the transfer instruction
    let ix = match transfer(
        &spl_token::id(),
        &source_ata,
        &destination_ata,
        &owner,
        &[&owner], // signers
        req.amount,
    ) {
        Ok(instruction) => instruction,
        Err(e) => {
            return Json(SendTokenResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to create transfer instruction: {}", e)),
            });
        }
    };

    // Convert account metas to response format
    let accounts = ix.accounts.iter().map(|meta| AccountInfo {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
    }).collect();

    // Encode instruction data
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