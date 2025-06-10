mod database;
mod handlers;
mod models;

use crate::models::Block;
use crate::models::Blockchain;
use crate::models::Transaction;

fn main() {
    print!("Testing Transaction model...");

    let tx = Transaction {
        tx_id: 1,
        sender: "0x1234".to_string(),
        receiver: "0x456def".to_string(),
        amount: 1000,
        timestamp: 12345567,
    };

    let tx2 = Transaction {
        tx_id: 2,
        sender: "0x2222".to_string(),
        receiver: "0x222def".to_string(),
        amount: 2000,
        timestamp: 22222222,
    };

    let tx_vec = vec![tx, tx2];

    let block = Block {
        block_id: 1,
        previous_block_hash: "zxd123".to_string(),
        timestamp: 123,
        tx_list: tx_vec,
        block_hash: "hdhh1234".to_string(),
    };

    println!("Block created: {:?}", block);
    println!("Blockchain API is ready!");

    let blockchain = Blockchain {
        chain_of_blocks: vec![block],
        total_tx: 2,
        metadata: "I love Rust".to_string(),
        current_block_height: 1,
    };

    println!("Block created: {:?}", blockchain);
    println!("Blockchain is up and running!");
}
