use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize)]
pub struct TransactionPayload{
   pub sol_to_send:String,
   pub to_pubkey:String
}