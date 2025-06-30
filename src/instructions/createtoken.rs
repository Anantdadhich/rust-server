use std::str::FromStr;
use axum::Json;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use solana_sdk::instruction::Instruction;
use spl_token::instruction::initialize_mint;
use base64::{engine::general_purpose, Engine as _};

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    #[serde(rename = "mintAuthority")]
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,

    
    #[serde(default)]
    pub token_name: Option<String>,
}

#[derive(Serialize)]
pub struct AccountInfo {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct CreateTokenData {
    pub program_id: String,
    pub accounts: Vec<AccountInfo>,
    pub instruction_data: String,
    pub token_name: Option<String>, 
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateTokenData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub async fn create_token(Json(payload): Json<CreateTokenRequest>) -> Json<CreateTokenResponse> {
   
    let mint_pubkey = match Pubkey::from_str(&payload.mint) {
        Ok(p) => p,
        Err(_) => {
            return Json(CreateTokenResponse {
                success: false,
                data: None,
                error: Some("Invalid mint public key".to_string()),
            });
        }
    };

    let authority_pubkey = match Pubkey::from_str(&payload.mint_authority) {
        Ok(p) => p,
        Err(_) => {
            return Json(CreateTokenResponse {
                success: false,
                data: None,
                error: Some("Invalid mint authority public key".to_string()),
            });
        }
    };

    let ix: Instruction = match initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &authority_pubkey,
        Some(&authority_pubkey), 
        payload.decimals,
    ) {
        Ok(i) => i,
        Err(e) => {
            return Json(CreateTokenResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to create instruction: {}", e)),
            });
        }
    };

    
    let accounts: Vec<AccountInfo> = ix
        .accounts
        .iter()
        .map(|acc| AccountInfo {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();

 
    let instruction_data = general_purpose::STANDARD.encode(&ix.data);

    Json(CreateTokenResponse {
        success: true,
        data: Some(CreateTokenData {
            program_id: ix.program_id.to_string(),
            accounts,
            instruction_data,
            token_name: payload.token_name.clone(),
        }),
        error: None,
    })
}
