use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
};

pub fn create_new_account() -> Result<([u8; 64], Pubkey), Box<dyn std::error::Error>> {
    // Generate a new random keypair
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey();
    let keypair_bytes = keypair.to_bytes();
    
    Ok((keypair_bytes, pubkey))
}