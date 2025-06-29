use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey
};
use std::str::FromStr;
use std::env;


pub fn get_balance(pubkey_str: &str) -> Result<f64, Box<dyn std::error::Error>> {
  let rcp_url = env::var("RCP_URL")?;
  let client: RpcClient = RpcClient::new_with_commitment(rcp_url.to_string(), CommitmentConfig::confirmed());

  let pubkey = Pubkey::from_str(pubkey_str)?;
  let balance_lamports = client.get_balance(&pubkey)?;

  return Ok(balance_lamports as f64 / LAMPORTS_PER_SOL as f64)
}