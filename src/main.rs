use axum::{routing::post, Router, http::Method};
use tower_http::cors::{Any, CorsLayer};

mod instructions;
mod model;


use instructions::{
    transfer::transfer_sol,
    message::{sign_message, verify_message},
};

use crate::instructions::{  createtoken::create_token, generatekeypair::generate_keypair, mint::mint_token, token::send_token};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(transfer_sol))
        .route("/send/token", post(send_token))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}