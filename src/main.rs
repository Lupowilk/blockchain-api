mod database;
mod handlers;
mod models;

use crate::models::Transaction;
use axum::{
    Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;

static TRANSACTIONS: Mutex<Vec<Transaction>> = Mutex::new(Vec::new());

async fn root() -> Json<serde_json::Value> {
    Json(json!(
        {
      "message": "Hello Blockchain API!",
      "status": "running",
      "version": "1.0"
    }
    ))
}

async fn get_transactions() -> Json<serde_json::Value> {
    let transaction = TRANSACTIONS.lock().unwrap().clone();
    Json(json!( {
        "transactions": transaction,
        "count": transaction.len()
    }))
}

async fn create_transaction(Json(payload): Json<Transaction>) -> Json<serde_json::Value> {
    TRANSACTIONS.lock().unwrap().push(payload.clone());
    Json(json!({
        "message":"Transaction created successfully",
        "transaction": payload
    }))
}

#[tokio::main]
async fn main() {
    // Router
    let user_request = Router::new()
        .route("/", get(root))
        .route("/transactions", get(get_transactions))
        .route("/transactions", post(create_transaction));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    //Start the server
    println!("Server starting on http://localhost:3000");
    axum::serve(listener, user_request).await.unwrap();
}
