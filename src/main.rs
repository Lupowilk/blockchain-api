mod database;
mod handlers;
mod models;

use crate::models::Block;
use crate::models::Blockchain;
use crate::models::Transaction;

use axum::serve;
use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

async fn root() -> String {
    "Hello Blockchian API".to_string()
}

#[tokio::main]
async fn main() {
    // Router
    let user_request = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    //Start the server
    println!("Server starting on http://localhost:3000");
    axum::serve(listener, user_request).await.unwrap();
}
