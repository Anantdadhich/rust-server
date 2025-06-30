use std::str::FromStr;
use axum::Json;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    system_program,
    sysvar::rent,
};
// Use the correct Pubkey type from solana-program
use solana_program::pubkey::Pubkey;
use spl_token::instruction::initialize_mint;
use base64::{engine::general_purpose, Engine as _};

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    #[serde(rename = "mintAuthority")]
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Serialize)]
pub struct AccountInfo {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateTokenData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct CreateTokenData {
    pub program_id: String,
    pub accounts: Vec<AccountInfo>,
    pub instruction_data: String,
}

 pub async fn create_token(
    Json(payload): Json<CreateTokenRequest>,
) -> Json<CreateTokenResponse> {
    // Validate and parse public keys
    let mint_pubkey = match Pubkey::from_str(&payload.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Json(CreateTokenResponse {
                success: false,
                data: None,
                error: Some("Invalid mint public key".to_string()),
            });
        }
    };

    let mint_authority_pubkey = match Pubkey::from_str(&payload.mint_authority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Json(CreateTokenResponse {
                success: false,
                data: None,
                error: Some("Invalid mint authority public key".to_string()),
            });
        }
    };

    // Create the initialize mint instruction
    let ix = match initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &mint_authority_pubkey,
        Some(&mint_authority_pubkey), // freeze_authority (optional)
        payload.decimals,
    ) {
        Ok(instruction) => instruction,
        Err(e) => {
            return Json(CreateTokenResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to create initialize mint instruction: {}", e)),
            });
        }
    };

    // Convert account metas to response format
    let accounts = ix.accounts.iter().map(|meta| AccountInfo {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
        is_writable: meta.is_writable,
    }).collect();

    // Encode instruction data
    let instruction_data = general_purpose::STANDARD.encode(&ix.data);

    Json(CreateTokenResponse {
        success: true,
        data: Some(CreateTokenData {
            program_id: ix.program_id.to_string(),
            accounts,
            instruction_data,
        }),
        error: None,
    })
}