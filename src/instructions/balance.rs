

use axum::Json;
use serde_json::json;
use solana_client::{rpc_client::RpcClient};
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey};



pub async  fn get_balance()->Json<serde_json::Value>{
    let client=RpcClient::new("	https://api.devnet.solana.com");

    let address=pubkey!("DG3B4NsaQVRg9AXwXEXrsXhkvt1t3ay9id7dtGEyeEv7");
     
     let balance=client.get_balance(&address).unwrap();
     
      Json(json!({
        "address": address.to_string(),
        "balance": (balance as f64) / (LAMPORTS_PER_SOL as f64)
    }))

     
      
    
}