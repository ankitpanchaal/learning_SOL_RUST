use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, 
    native_token::LAMPORTS_PER_SOL, 
    pubkey::Pubkey, 
    signature::Signature
};
use std::env;
use std::str::FromStr;
use tokio::time::{sleep, Duration};

pub async fn airdrop_sol(pubkey_str: &str, amount: f64) -> Result<Signature, Box<dyn std::error::Error + Send + Sync>> {
    let rcp_url = env::var("RCP_URL")?;

    let client = RpcClient::new_with_commitment(rcp_url.to_string(), CommitmentConfig::confirmed());
    let pubkey = Pubkey::from_str(pubkey_str)?;

    println!("Requesting airdrop for pubkey: {}", pubkey);

    let amt = (amount * (LAMPORTS_PER_SOL as f64)) as u64;
    let signature = client.request_airdrop(&pubkey, amt)?;

    println!("Airdrop signature: {}", signature);
    println!("Track on Solana Explorer: https://explorer.solana.com/tx/{}?cluster=devnet", signature);

    loop {
        let confirmed = client.confirm_transaction(&signature)?;
        if confirmed {
            println!("Airdrop confirmed!");
            break;
        }
        sleep(Duration::from_millis(500)).await;
    }

    Ok(signature)
}