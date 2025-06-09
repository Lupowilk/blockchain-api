use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub tx_id: u64,
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub timestamp: u64,
}
