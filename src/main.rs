mod utils;
mod api;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let app = Router::new()
        .route("/api/account/create", get(api::handlers::create_account))
        .route("/api/balance/:pubkey", get(api::handlers::get_balance))
        .route("/api/airdrop", post(api::handlers::airdrop))
        .route("/api/transfer", post(api::handlers::transfer))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app)
        .await
        .unwrap();
}