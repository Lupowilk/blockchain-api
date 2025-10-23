mod database;
mod handlers;
mod models;

use crate::handlers::transaction;
use crate::models::Transaction;
use axum::{
    Json, Router,
    routing::{delete, get, post},
};

use mongodb::{Client, bson::doc, IndexModel};
use serde_json::json;
use utoipa::{OpenApi};
use crate::models::transaction::CreateTransactionInput;
use utoipa_swagger_ui::SwaggerUi;

use tracing_subscriber::EnvFilter;
use tracing::info;

#[derive(OpenApi)]
#[openapi(
    paths(
        transaction::create_transaction,
        transaction::get_transactions,
        transaction::get_transaction_by_id,
        transaction::delete_transaction_by_id
    ),
    components(
        schemas(Transaction, CreateTransactionInput)
    ),
    tags(
        (name = "transaction", description = "Transaction management endpoints")
    )
)]
struct ApiDoc;


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
    //Initialize tracing subscriber
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Client setup
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("Failed to connect to MongoDB");


    // Indexes for sender, receiver, amount and timestamps
    let db = client.database("blockchain");
    let collection = db.collection::<Transaction>("transactions");

    // sender
    let sender_index = IndexModel::builder()
        .keys(doc! {"sender": 1})
        .build();

    collection
        .create_index(sender_index)
        .await
        .expect("Failed to create sender index.");

    info!("Created sender index");

    // receiver
    let receiver_index = IndexModel::builder()
        .keys(doc! {"receiver": 1})
        .build();

    collection
        .create_index(receiver_index)
        .await
        .expect("Failed to created receiver index");

    info!("Created the receiver index");

    //amount
    let amount_index = IndexModel::builder()
        .keys(doc! {"amount": 1})
        .build();

    collection
        .create_index(amount_index)
        .await
    .expect("Failder to created amount index");

    info!("Created amount index");

    //timestamp
    let timestamp_index = IndexModel::builder()
        .keys(doc! {"timestamp": -1})
        .build();

    collection
        .create_index(timestamp_index)
        .await
        .expect("Failed to create timestamp index");

    info!("Created timestamp index");


    // Router setup
    let user_request = Router::new()
        // stores the client in Axum's state manager
        .route("/", get(root))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
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
    info!("Server starting on http://localhost:3000");
    axum::serve(listener, user_request).await.unwrap();
}
