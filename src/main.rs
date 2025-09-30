mod database;
mod handlers;
mod models;

use crate::handlers::transaction;
use crate::models::Transaction;
use axum::{
    Json, Router,
    routing::{delete, get, post},
};

use mongodb::{Client, bson::doc};
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

#[tokio::main]
async fn main() {
    // Client sdetup
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("Failed to connect to MongoDB");

    // Router setup
    let user_request = Router::new()
        // stores the client in Axum's state manager
        .route("/", get(root))
        .route("/transactions", get(transaction::get_transactions))
        .route("/transactions", post(transaction::create_transaction))
        .route(
            "/transactions/{id}",
            get(transaction::get_transaction_by_id),
        )
        .route(
            "/transactions/{id}",
            delete(transaction::delete_transaction_by_id),
        )
        .with_state(client); // Changes Router<Client> to Router<>, stores the client in Axum's state manager

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    //Start the server
    println!("Server starting on http://localhost:3000");
    axum::serve(listener, user_request).await.unwrap();
}
