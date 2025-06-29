use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    system_instruction,
    transaction::Transaction,
    signer::Signer,
};
use std::env;
use std::str::FromStr;

pub fn transfer_sol(
    from_keypair: &Keypair,
    to_pubkey_str: &str,
    amount: f64,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let rcp_url = env::var("RCP_URL")?;
    let client = RpcClient::new_with_commitment(rcp_url.to_string(), CommitmentConfig::confirmed());
    
    // Convert string to Pubkey
    let to_pubkey = Pubkey::from_str(to_pubkey_str)?;
    let from_pubkey = from_keypair.pubkey();
    
    println!("Transferring {} SOL from {} to {}", amount, from_pubkey, to_pubkey);
    
    // Convert SOL to lamports
    let lamports = (amount * LAMPORTS_PER_SOL as f64) as u64;
    
    // Create transfer instruction
    let transfer_instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, lamports);
    
    // Get recent blockhash
    let recent_blockhash = client.get_latest_blockhash()?;
    
    // Create transaction
    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&from_pubkey),
        &[from_keypair],
        recent_blockhash,
    );
    
    // Send transaction
    let signature = client.send_and_confirm_transaction(&transaction)?;
    
    println!("Transfer signature: {}", signature);
    println!("Track on Solana Explorer: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    
    Ok(signature)
}