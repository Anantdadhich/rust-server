use anyhow::Result;
use axum::response::IntoResponse;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, signature::Signer,
    signer::keypair::Keypair, system_instruction, transaction::Transaction,
};

#[tokio::main] 
 pub async  fn transfer()->Result<()>{


    let client=RpcClient::new("https://api.devnet.solana.com".to_string());

    let sender=Keypair::new();
    let recipient=Keypair::new();

    let airdrop_sig=client.request_airdrop(&sender.pubkey(), LAMPORTS_PER_SOL).await?; 

    loop {
        let confirm=client.confirm_transaction(&airdrop_sig).await?;
        if confirm{
            break;
        }
    }
    //let check balance before transfer
    let sender_balance=client.get_balance(&sender.pubkey()).await?; 
    let recipient_balance=client.get_balance(&recipient.pubkey()).await?;

    let transfer_amount=LAMPORTS_PER_SOL/100;

    let transfer_instruction=system_instruction::transfer(&sender.pubkey(), &recipient.pubkey(), transfer_amount) ;
     

     let mut transaction=Transaction::new_with_payer(&[transfer_instruction], Some(&sender.pubkey()));


     let blockhash=client.get_latest_blockhash().await?;

     transaction.sign(&[&sender], blockhash);

     let transaction_sig=client.send_and_confirm_transaction(&transaction).await?;

     println!("Transaction done{:#?}",transaction_sig);

     
    Ok(())

}