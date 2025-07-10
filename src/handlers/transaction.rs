use crate::models::Transaction;
use axum::Json;
use axum::extract::Path;
use futures_util::StreamExt;
use mongodb::{Client, bson::doc};
use serde_json::json;

//A fucntion that returens all stored transacions
pub async fn get_transactions() -> Json<serde_json::Value> {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let database = client.database("blockchain");
    let collection = database.collection("transactions");
    let mut cursor = collection.find(doc! {}).await.unwrap();
    let mut transaction_data: Vec<Transaction> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(transaction) => transaction_data.push(transaction),
            Err(_) => break,
        }
    }

    Json(json!( {
        "transactions":transaction_data,
        "count": transaction_data.len()
    }))
}

// New function for getting single transaction by ID
pub async fn get_transaction_by_id(Path(id): Path<String>) -> Json<serde_json::Value> {
    Json(json!({
        "message": "You requested transaction ID",
        "id": id
    }))
}
