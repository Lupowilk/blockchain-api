mod database;
mod handlers;
mod models;

use crate::models::{Transaction, blockchain};
use axum::{
    Json, Router,
    routing::{get, post},
};
use mongodb::bson::doc;
use mongodb::{Client, Collection, Database};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;

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
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let database = client.database("blockchain");
    let transactions = database.collection("transactions");

    Json(json!( {
        "transactions": ,
        "count": t
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
