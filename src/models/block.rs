use crate::models::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub block_id: u64,
    pub previous_block_hash: String,
    pub timestamp: u64,
    pub tx_list: Vec<Transaction>,
    pub block_hash: String,
}
