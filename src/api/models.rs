use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CreateAccountResponse {
    pub pubkey: String,
    pub keypair: Vec<u8>,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub pubkey: String,
    pub balance: f64,
}

#[derive(Deserialize)]
pub struct AirdropRequest {
    pub pubkey: String,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct AirdropResponse {
    pub signature: String,
    pub pubkey: String,
    pub amount: f64,
}

#[derive(Deserialize)]
pub struct TransferRequest {
    pub from_keypair: Vec<u8>,
    pub to_pubkey: String,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct TransferResponse {
    pub signature: String,
    pub from_pubkey: String,
    pub to_pubkey: String,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}