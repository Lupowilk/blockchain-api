use crate::models::Block;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub chain_of_blocks: Vec<Block>,
    pub total_tx: u64,
    pub metadata: String,
    pub current_block_height: u64,
}
