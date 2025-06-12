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
    // Create sample transactions
    let tx1 = Transaction {
        tx_id: 1,
        sender: "0x123abc".to_string(),
        receiver: "0x456def".to_string(),
        amount: 1000,
        timestamp: 1234567890,
    };

    let tx2 = Transaction {
        tx_id: 2,
        sender: "0x789ghi".to_string(),
        receiver: "0x012jkl".to_string(),
        amount: 2500,
        timestamp: 1234567900,
    };

    Json(json!( {
        "transactions": [tx1, tx2],
        "count": 2
    }))
}

async fn create_transaction(Json(payload): Json<Transaction>) -> Json<serde_json::Value> {
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
