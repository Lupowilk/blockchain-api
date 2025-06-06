mod database;
mod handlers;
mod models;

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

    println!("Transaction created: {:?}", tx);
    println!("Blockchain API is ready!");
}
