use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
};
use solana_sdk::signature::{Keypair, Signer};
use crate::utils::{
    account::create_new_account,
    get_balance::get_balance as get_balance_util,
    airdrop::airdrop_sol,
    transfer::transfer_sol,
};
use super::models::*;

pub async fn create_account() -> Result<Json<CreateAccountResponse>, (StatusCode, Json<ErrorResponse>)> {
    match create_new_account() {
        Ok((keypair_bytes, pubkey)) => {
            Ok(Json(CreateAccountResponse {
                pubkey: pubkey.to_string(),
                keypair: keypair_bytes.to_vec(),
            }))
        }
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to create account: {}", e),
                }),
            ))
        }
    }
}

pub async fn get_balance(
    Path(pubkey): Path<String>,
) -> Result<Json<BalanceResponse>, (StatusCode, Json<ErrorResponse>)> {
    match get_balance_util(&pubkey).await {
        Ok(balance) => {
            Ok(Json(BalanceResponse {
                pubkey,
                balance,
            }))
        }
        Err(e) => {
            Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Failed to get balance: {}", e),
                }),
            ))
        }
    }
}

pub async fn airdrop(
    Json(payload): Json<AirdropRequest>,
) -> Result<Json<AirdropResponse>, (StatusCode, Json<ErrorResponse>)> {
    match airdrop_sol(&payload.pubkey, payload.amount).await {
        Ok(signature) => {
            Ok(Json(AirdropResponse {
                signature: signature.to_string(),
                pubkey: payload.pubkey,
                amount: payload.amount,
            }))
        }
        Err(e) => {
            Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Airdrop failed: {}", e),
                }),
            ))
        }
    }
}

pub async fn transfer(
    Json(payload): Json<TransferRequest>,
) -> Result<Json<TransferResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Convert bytes back to Keypair
    let keypair_bytes: [u8; 64] = match payload.from_keypair.try_into() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid keypair format".to_string(),
                }),
            ));
        }
    };
    
    let from_keypair = match Keypair::from_bytes(&keypair_bytes) {
        Ok(kp) => kp,
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Failed to reconstruct keypair: {}", e),
                }),
            ));
        }
    };
    
    let from_pubkey = from_keypair.pubkey().to_string();
    
    match transfer_sol(&from_keypair, &payload.to_pubkey, payload.amount).await {
        Ok(signature) => {
            Ok(Json(TransferResponse {
                signature: signature.to_string(),
                from_pubkey,
                to_pubkey: payload.to_pubkey,
                amount: payload.amount,
            }))
        }
        Err(e) => {
            Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Transfer failed: {}", e),
                }),
            ))
        }
    }
}