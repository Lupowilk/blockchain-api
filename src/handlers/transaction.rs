use crate::models::Transaction;
use axum::Json;
use axum::extract::Path;
use futures_util::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::raw::RawArrayIter;
use mongodb::{Client, bson::doc};
use serde_json::json;
use tokio::task::Id;

//A function that returens all stored transacions
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
    // Convert string to ObjectId
    let object_id = match ObjectId::parse_str(id) {
        Ok(id) => id, // Succces - use the ObjectID
        Err(_) => {
            return Json(json!({
                "error": "HTTP 400 -invalid ID format"
            }));
        }
    };

    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let database = client.database("blockchain");
    let collection: mongodb::Collection<Transaction> = database.collection("transactions");
    let transaction_by_id = match collection.find_one(doc! {"_id": object_id}).await.unwrap() {
        Some(transaction) => transaction,
        None => {
            return Json(json!({
                "error": "HTTP 404 - no transaction found"
            }));
        }
    };

    Json(json!({
        "transaction": transaction_by_id
    }))
}

pub async fn delete_transaction_by_id(Path(id): Path<String>) -> Json<serde_json::Value> {
    let object_id = match ObjectId::parse_str(id) {
        Ok(id) => id,
        Err(_) => {
            return Json(json!({
                "error": "Invalid ID format, please provide this format: 685ba45cb808dcc5709476a2"
            }));
        }
    };

    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let database = client.database("blockchain");
    let collection = database.collection("transactions");

    Json(json!({"message": "TODO: implement delete logic"}))
}
