use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, program_pack::Pack,
    pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction::create_account,
    transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};
use spl_token::{
    instruction::{initialize_mint2, mint_to_checked, transfer_checked},
    state::Mint,
    ID as TOKEN_PROGRAM_ID,
};

#[tokio::main]
async fn send_token() -> anyhow::Result<()> {
    let client = RpcClient::new_with_commitment(
        String::from("http://127.0.0.1:8899"),
        CommitmentConfig::confirmed(),
    );

    let sender = Keypair::new(); // will also act as tx fee payer
    let recipient = Keypair::new();
    let mint_account = Keypair::new();
    let sender_token_account =
        get_associated_token_address(&sender.pubkey(), &mint_account.pubkey());
    let recipient_token_account =
        get_associated_token_address(&recipient.pubkey(), &mint_account.pubkey());

    // Airdrop sender, create the mint account, create ATAs for sender and recipient, mint tokens to sender
    setup(
        &client,
        &sender,
        &recipient,
        &mint_account,
        &sender_token_account,
    )
    .await?;

    let decimals = client
        .get_token_account_balance(&sender_token_account)
        .await?
        .decimals;
    let transfer_amount = 1 * 10_u64.pow(decimals as u32); // 1 token

    let transfer_ix = transfer_checked(
        &TOKEN_PROGRAM_ID,
        &sender_token_account,
        &mint_account.pubkey(),
        &recipient_token_account,
        &sender.pubkey(),
        &[&sender.pubkey()],
        transfer_amount,
        decimals,
    )?;

    let mut transaction = Transaction::new_with_payer(&[transfer_ix], Some(&sender.pubkey()));

    transaction.sign(&[&sender], client.get_latest_blockhash().await?);

    match client.send_and_confirm_transaction(&transaction).await {
        Ok(signature) => println!("Transaction Signature: {}", signature),
        Err(err) => eprintln!("Error transferring tokens: {}", err),
    }

    Ok(())
}

// Helper function to airdrop sender, create the mint account
// create ATAs for sender and recipient, mint tokens to sender token account
async fn setup(
    client: &RpcClient,
    sender: &Keypair,
    recipient: &Keypair,
    mint_account: &Keypair,
    sender_token_account: &Pubkey,
) -> anyhow::Result<()> {
    let transaction_signature = client
        .request_airdrop(&sender.pubkey(), 5 * LAMPORTS_PER_SOL)
        .await?;
    loop {
        if client.confirm_transaction(&transaction_signature).await? {
            break;
        }
    }

    let decimals = 9;
    let amount_to_mint = 1 * 10_u64.pow(decimals as u32); // 1 token
    let mint_account_len = Mint::LEN;
    let mint_account_rent = client
        .get_minimum_balance_for_rent_exemption(mint_account_len)
        .await?;

    let create_mint_account_ix = create_account(
        &sender.pubkey(),
        &mint_account.pubkey(),
        mint_account_rent,
        mint_account_len as u64,
        &TOKEN_PROGRAM_ID,
    );

    let initialize_mint_ix = initialize_mint2(
        &TOKEN_PROGRAM_ID,
        &mint_account.pubkey(),
        &sender.pubkey(),
        Some(&sender.pubkey()),
        decimals,
    )?;

    // Create ATA instruction
    let create_sender_ata_ix = create_associated_token_account_idempotent(
        &sender.pubkey(),       // payer
        &sender.pubkey(),       // wallet address
        &mint_account.pubkey(), // mint address
        &TOKEN_PROGRAM_ID,
    );

    let create_recipient_ata_ix = create_associated_token_account_idempotent(
        &sender.pubkey(),       // payer
        &recipient.pubkey(),    // wallet address
        &mint_account.pubkey(), // mint address
        &TOKEN_PROGRAM_ID,
    );

    let mint_to_ix = mint_to_checked(
        &TOKEN_PROGRAM_ID,
        &mint_account.pubkey(),
        &sender_token_account,
        &sender.pubkey(),
        &[&sender.pubkey()],
        amount_to_mint,
        decimals,
    )?;

    let mut transaction = Transaction::new_with_payer(
        &[
            create_mint_account_ix,
            initialize_mint_ix,
            create_sender_ata_ix,
            create_recipient_ata_ix,
            mint_to_ix,
        ],
        Some(&sender.pubkey()),
    );

    transaction.sign(
        &[&sender, &mint_account],
        client.get_latest_blockhash().await?,
    );

    client.send_and_confirm_transaction(&transaction).await?;
    Ok(())
}