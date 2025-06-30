mod utils;

use solana_sdk::signer::Signer;
use utils::airdrop::airdrop_sol;
use utils::get_balance::get_balance;
use utils::transfer::transfer_sol;
use solana_sdk::signature::read_keypair_file;
use utils::account::create_new_account;

fn main() {
    dotenv::dotenv().ok();
    // match create_new_account() {
    //     Ok((keypair, pubkey)) => {
    //         println!("Generated new account:");
    //         println!("Pubkey: {}", pubkey);
    //         println!("KEYpair: {:?}", keypair);
    //     }
    //     Err(e) => {
    //         println!("Failed to create new account: {}", e);
    //     }
    // }


    // Load first wallet (sender)
    // let keypair1 = read_keypair_file("./devnet-keypair.json")
    //     .expect("Failed to load keypair1");
    // let pubkey1 = keypair1.try_pubkey().expect("Failed to get pubkey from keypair1");
    
    // // Load second wallet (receiver)
    // let keypair2 = read_keypair_file("./devnet-keypair2.json")
    //     .expect("Failed to load keypair2");
    // let pubkey2 = keypair2.try_pubkey().expect("Failed to get pubkey from keypair2");
    
    // println!("=== WALLET 1 (Sender) ===");
    // println!("Pubkey: {}", pubkey1.to_string());
    // println!("View account: https://explorer.solana.com/address/{}?cluster=devnet", pubkey1);
    
    // println!("\n=== WALLET 2 (Receiver) ===");
    // println!("Pubkey: {}", pubkey2.to_string());
    // println!("View account: https://explorer.solana.com/address/{}?cluster=devnet", pubkey2);
    
    // Airdrop to wallet 1 first
    // println!("\n=== AIRDROPPING TO WALLET 1 ===");
    // match airdrop_sol(&pubkey1.to_string(), 5.0) {
    //     Ok(signature) => {
    //         println!("Airdrop completed successfully!");
    //         println!("Transaction: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    //     },
    //     Err(e) => println!("Airdrop failed: {}", e),
    // }
    
    // println!("\n=== CHECKING BALANCES BEFORE TRANSFER ===");
    // match get_balance(&pubkey1.to_string()) {
    //     Ok(balance) => println!("Wallet 1 balance: {} SOL", balance),
    //     Err(e) => println!("Failed to get wallet 1 balance: {}", e),
    // }
    
    // match get_balance(&pubkey2.to_string()) {
    //     Ok(balance) => println!("Wallet 2 balance: {} SOL", balance),
    //     Err(e) => println!("Failed to get wallet 2 balance: {}", e),
    // }
    
    // // Transfer from wallet 1 to wallet 2
    // println!("\n=== TRANSFERRING 2 SOL FROM WALLET 1 TO WALLET 2 ===");
    // match transfer_sol(&keypair1, &pubkey2.to_string(), 0.2) {
    //     Ok(signature) => {
    //         println!("Transfer completed successfully!");
    //         println!("Transaction: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    //     },
    //     Err(e) => println!("Transfer failed: {}", e),
    // }
    
    // // Check balances after transfer
    // println!("\n=== CHECKING BALANCES AFTER TRANSFER ===");
    // match get_balance(&pubkey1.to_string()) {
    //     Ok(balance) => println!("Wallet 1 balance: {} SOL", balance),
    //     Err(e) => println!("Failed to get wallet 1 balance: {}", e),
    // }
    
    // match get_balance(&pubkey2.to_string()) {
    //     Ok(balance) => println!("Wallet 2 balance: {} SOL", balance),
    //     Err(e) => println!("Failed to get wallet 2 balance: {}", e),
    // }
}