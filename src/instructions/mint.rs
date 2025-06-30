use std::str::FromStr;
use axum::Json;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use spl_token::instruction::mint_to;
use spl_associated_token_account::get_associated_token_address;
use base64::{engine::general_purpose, Engine as _};

#[derive(Deserialize)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct AccountInfo {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct MintTokenResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<MintTokenData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct MintTokenData {
    pub program_id: String,
    pub accounts: Vec<AccountInfo>,
    pub instruction_data: String,
}

pub async fn mint_token(
    Json(payload): Json<MintTokenRequest>,
) -> Json<MintTokenResponse> {
    // Validate amount
    if payload.amount == 0 {
        return Json(MintTokenResponse {
            success: false,
            data: None,
            error: Some("Amount must be greater than 0".to_string()),
        });
    }

    // Parse and validate mint address
    let mint_pubkey = match Pubkey::from_str(&payload.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Json(MintTokenResponse {
                success: false,
                data: None,
                error: Some("Invalid mint public key".to_string()),
            });
        }
    };

    // Parse and validate destination address
    let destination_pubkey = match Pubkey::from_str(&payload.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Json(MintTokenResponse {
                success: false,
                data: None,
                error: Some("Invalid destination public key".to_string()),
            });
        }
    };

    // Parse and validate authority address
    let authority_pubkey = match Pubkey::from_str(&payload.authority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Json(MintTokenResponse {
                success: false,
                data: None,
                error: Some("Invalid authority public key".to_string()),
            });
        }
    };

    // Get the associated token account for the destination
    let destination_ata = get_associated_token_address(&destination_pubkey, &mint_pubkey);

    // Create the mint_to instruction
    let ix = match mint_to(
        &spl_token::id(),
        &mint_pubkey,
        &destination_ata,
        &authority_pubkey,
        &[&authority_pubkey], // signers
        payload.amount,
    ) {
        Ok(instruction) => instruction,
        Err(e) => {
            return Json(MintTokenResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to create mint instruction: {}", e)),
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

    Json(MintTokenResponse {
        success: true,
        data: Some(MintTokenData {
            program_id: ix.program_id.to_string(),
            accounts,
            instruction_data,
        }),
        error: None,
    })
}