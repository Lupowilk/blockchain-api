use crate::models::transaction::{CreateTransactionInput, Transaction};
use crate::models::TransactionQuery;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use futures_util::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, bson::doc};
use mongodb::options::FindOptions;
use rand::Rng;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

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
#[utoipa::path(
    post,
    path = "/transactions",
    request_body = CreateTransactionInput,
    responses(
        (status = 200, description = "Transaction created successfully"),
        (status = 400, description = "invalid input")
    )
)]
pub async fn create_transaction(State(client): State<Client>, Json(payload): Json<CreateTransactionInput>) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {

    // Amount validation
    if payload.amount == 0 {
         return Err(AppError::BadRequest("Amount must be greater than 0".to_string()));
    }

    // Sender validation
    if payload.sender.trim().is_empty() == true {
        return Err(AppError::BadRequest("You must provide the sender address".to_string()));
    }

    // Receiver validation
    if payload.receiver.trim().is_empty() == true {
        return Err(AppError::BadRequest("Please provide a receiver address".to_string()));
    }

    // Same address validation
    if payload.sender.trim() == payload.receiver.trim() {
        return Err(AppError::BadRequest("Cannot send to yourself".to_string()));
    }
    // This code only runs if amount is not 0
    let database = client.database("blockchain");
    let collection = database.collection("transactions");

    // 1. Generate system fields
    let id = ObjectId::new();
    let tx_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::Database(format!("System clock error: {}",e)))?
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
        .map_err(|e| AppError::Database(format!("Failed to save transaction: {}", e)))?;

    Ok((StatusCode::OK, Json(json!({
        "message":"Transaction created successfully",
        "transaction": payload
    }))))
}

//A function that returens all stored transacions
#[utoipa::path(
    get,
    path = "/transactions",
    responses(
        (status = 200, description = "List of transactions retrieved successfully")
    )
)]
pub async fn get_transactions(State(client): State<Client>, Query(params): Query<TransactionQuery>) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let database = client.database("blockchain");
    let collection = database.collection("transactions");

    // Step 1: Get limit, offset and filters
    let limit = params.get_limit();
    let offset = params.get_offset();
    let mut filter = doc! {};

    if let Some(sender) = &params.sender {
            filter.insert("sender", sender);
    }

    if let Some(receiver) = &params.receiver {
            filter.insert("receiver", receiver);
    }

    if let Some(amount) = &params.amount {
            filter.insert("amount", amount);
    }


    // Step 2: Build FindOptions
    let find_options = FindOptions::builder()
        .limit(limit as i64)
        .skip(offset)
        .build();

    // Step 3: Use them in find
    let mut cursor = collection
        .find(filter)
        .with_options(find_options)
        .await
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
#[utoipa::path(
    get,
    path = "/transactions/{id}",
    responses(
        (status = 200, description = "Transaction found"),
        (status = 400, description = "invalid ID format, please provide 13612637127132"),
        (status = 404, description = "Transaction not found")
    )
)]
pub async fn get_transaction_by_id(State(client): State<Client>, Path(id): Path<String>) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let object_id = match ObjectId::parse_str(id) {
        Ok(id) => id, // Succces - use the ObjectID
        Err(_) => {
            return Err(AppError::BadRequest("Invalid ID format".to_string()));
        }
    };
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
#[utoipa::path(
    delete,
    path = "/transactions/{id}",
    responses(
        (status = 200, description = "Transaction deleted successfully"),
        (status = 400, description = "invalid ID format, please provide 13612637127132"),
        (status = 404, description = "Transaction not found")
    )
)]
pub async fn delete_transaction_by_id(State(client): State<Client>, Path(id): Path<String>) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let object_id = match ObjectId::parse_str(id) {
        Ok(id) => id,
        Err(_) => {
            return Err(AppError::BadRequest("Invalid ID format, please provide this format: 685ba45cb808dcc5709476a2".to_string()));
        }
    };
    let database = client.database("blockchain");
    let collection = database.collection::<Transaction>("transactions");
    let transaction_id = collection.delete_one(doc! {"_id":object_id}).await
        .map_err(|e| AppError::Database(format!("Failed to delete transaction: {}", e)))?;

    if transaction_id.deleted_count == 1 {
        Ok((StatusCode::OK, Json(json!({"message": "Transaction deleted succesfully."}))))
    } else {
        Err(AppError::NotFound("Transaction not found.".to_string()))
    }
}
