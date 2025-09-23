use crate::models::transaction::{CreateTransactionInput, Transaction};
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use futures_util::future::OrElse;
use futures_util::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::raw::RawArrayIter;
use mongodb::{Client, bson::doc};
use rand::Rng;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::task::Id;
use uuid::timestamp;

//Enum for handling errors
pub enum AppError {
    Database(String),
    NotFound(String),
    BadRequest(String),
}

//IntoResponse implementation for AppError
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Database(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": msg}))).into_response()
            }
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, Json(json!({"message": msg}))).into_response()
            }
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, Json(json!({"message": msg}))).into_response()
            }
        }
    }
}

// A function that takes a transaction from a user and saves it permanently to MongoDB
pub async fn create_transaction(Json(payload): Json<CreateTransactionInput>) -> (StatusCode, Json<serde_json::Value>) {

    // Amount validation
    if payload.amount == 0 {
         return (StatusCode::BAD_REQUEST, Json(json!({
             "message":"Amount must be greater than 0"
         })))
    }

    // Sender validation
    if payload.sender.trim().is_empty() == true {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "message":"You must provide the sender address"
        })))
    }

    // Receiver validation
    if payload.receiver.trim().is_empty() == true {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "message": "Please provide a receiver address"
        })))
    }

    // Same address validation
    if payload.sender.trim() == payload.receiver.trim() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "message": "Cannot send to yourself"
        })))
    }
    // This code only runs if amount is not 0
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
    let tx_id = tx_timestamp * 1000 + random_component;

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

    (StatusCode::OK, Json(json!({
        "message":"Transaction created successfully",
        "transaction": payload
    })))
}



//A function that returens all stored transacions
pub async fn get_transactions() -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .map_err(|e| AppError::Database(format!("Failed to connect to MongoDB: {}", e)))?;
    let database = client.database("blockchain");
    let collection = database.collection("transactions");
    let mut cursor = collection.find(doc! {}).await
        .map_err(|e| AppError::Database(format!("Failed to query transaction: {}", e)))?;
    let mut transaction_data: Vec<Transaction> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(transaction) => transaction_data.push(transaction),
            Err(_) => break,
        }
    }

    Ok((StatusCode::OK, Json(json!( {
        "transactions":transaction_data,
        "count": transaction_data.len()
    }))))
}




// A function that returns a trasaction based on ID.
pub async fn get_transaction_by_id(Path(id): Path<String>) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let object_id = match ObjectId::parse_str(id) {
        Ok(id) => id, // Succces - use the ObjectID
        Err(_) => {
            return Err(AppError::BadRequest("Invalid ID format".to_string()));
        }
    };
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .map_err(|e| AppError::Database(format!("Failed to conncet to MongoDB: {}", e)))?;
    let database = client.database("blockchain");
    let collection: mongodb::Collection<Transaction> = database.collection("transactions");
    let transaction_by_id = match collection.find_one(doc! {"_id": object_id}).await
        .map_err(|e| AppError::Database(format!("Failed to query transaction: {}", e)))? {
        Some(transaction) => transaction,
        None => {
            return Err(AppError::NotFound("Transaction not found".to_string()));
        }
    };

    Ok ((StatusCode::OK, Json(json!({
        "transaction": transaction_by_id
    }))))
}

//A function that delets a transaction by ID.
pub async fn delete_transaction_by_id(Path(id): Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    let object_id = match ObjectId::parse_str(id) {
        Ok(id) => id,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "error": "Invalid ID format, please provide this format: 685ba45cb808dcc5709476a2"
            })));
        }
    };
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let database = client.database("blockchain");
    let collection = database.collection::<Transaction>("transactions");
    let transaction_id = collection.delete_one(doc! {"_id":object_id}).await.unwrap();

    if transaction_id.deleted_count == 1 {
        (StatusCode::OK, Json(json!({"message": "Transaction deleted succesfully."})))
    } else {
        (StatusCode::NOT_FOUND, Json(json!({"message": "Transaction not found."})))
    }
}
