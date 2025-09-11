use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub tx_id: u64,
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateTransactionInput {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}
