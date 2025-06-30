mod utils;
mod api;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let app = Router::new()
        .route("/api/account/create", get(api::handlers::create_account))
        .route("/api/balance/:pubkey", get(api::handlers::get_balance))
        .route("/api/airdrop", post(api::handlers::airdrop))
        .route("/api/transfer", post(api::handlers::transfer))
        .layer(CorsLayer::permissive());

    // Get port from environment or default to 3000
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let bind_address = format!("0.0.0.0:{}", port);
    
    let listener = TcpListener::bind(&bind_address).await.unwrap();
    println!("Server running on http://{}", bind_address);
    
    axum::serve(listener, app)
        .await
        .unwrap();
}