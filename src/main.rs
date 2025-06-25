mod database;
mod handlers;
mod models;

use std::collections;

use crate::models::Transaction;
use axum::{
    Json, Router,
    routing::{get, post},
};

use mongodb::{Client, Collection, Database};
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

// async fn get_transactions() -> Json<serde_json::Value> {
//     let client = Client::with_uri_str("mongodb://localhost:27017")
//         .await
//         .unwrap();
//     let database = client.database("blockchain");
//     let collection = database.collection("transactions");
//     let cursor = collection.find(None, None).await.unwrap();

//     Json(json!( {
//         "transactions": ,
//         "count": t
//     }))
// }

// A fucntion that takes a transaction from a user and saves it permanently to MongoDB
async fn create_transaction(Json(payload): Json<Transaction>) -> Json<serde_json::Value> {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let database = client.database("blockchain");
    let collection = database.collection("transactions");
    let new_transaction = collection.insert_one(payload.clone()).await.unwrap();

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
        // .route("/transactions", get(get_transactions))
        .route("/transactions", post(create_transaction));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    //Start the server
    println!("Server starting on http://localhost:3000");
    axum::serve(listener, user_request).await.unwrap();
}
