use crate::models::transaction::{CreateTransactionInput, Transaction};
use axum::Json;
use axum::extract::Path;
use futures_util::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::raw::RawArrayIter;
use mongodb::{Client, bson::doc};
use rand::Rng;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::task::Id;
use uuid::timestamp;

// A function that takes a transaction from a user and saves it permanently to MongoDB
pub async fn create_transaction(
    Json(payload): Json<CreateTransactionInput>,
) -> Json<serde_json::Value> {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let database = client.database("blockchain");
    let collection = database.collection("transactions");

    // 1. Generate system fields
    let id = ObjectId::new();
    let tx_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let random_component = rand::thread_rng().gen_range(0..1000);
    let tx_id = tx_timestamp + random_component;

    // 2. Build the Transaction
    let transaction_to_save = Transaction {
        id: id,
        tx_id: tx_id,
        sender: payload.sender.clone(),
        receiver: payload.receiver.clone(),
        amount: payload.amount,
        timestamp: tx_timestamp,
    };
    // 3. Insert the biult transaction
    collection
        .insert_one(transaction_to_save.clone())
        .await
        .unwrap();

    Json(json!({
        "message":"Transaction created successfully",
        "transaction": payload
    }))
}

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
    let collection = database.collection::<Transaction>("transactions");
    let transaction_id = collection.delete_one(doc! {"_id":object_id}).await.unwrap();

    if transaction_id.deleted_count == 1 {
        Json(json!({"message": "HTTP 204 No content. The trasaction was removed succesfully."}))
    } else {
        Json(
            json!({"message": "HTTP 404 there is an error. The trasaction was not removed succesfully."}),
        )
    }
}
