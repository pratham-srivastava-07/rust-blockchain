use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

// struct for transactions
#[derive(Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32
}
// struct for one block
#[derive(Debug)]
struct Block {
    timestamp: DateTime<Utc>,
    previous_hash: Vec<u8>,
    hash: Vec<u8>,
    transactions: Vec<Transaction>
}

impl Block {
    fn calculate_hash(&mut self) -> Vec<u8> {
        let mut new_hash = Sha256::new();

        let timestamp = self.timestamp.to_string().into_bytes();
        let previous_hash = format!("{:?}", self.previous_hash).into_bytes();
        let hash = format!("{:?}", self.hash).into_bytes();
        let transaction_bytes = format!("{:?}", self.transactions).into_bytes();

        new_hash.update(timestamp);
        new_hash.update(transaction_bytes);
        new_hash.update(previous_hash);

        new_hash.finalize().to_vec()
    }

    fn mine_block(&mut self) {}
}
// struct for blockchain
#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>
}

fn main() {
    println!("Hello, world!");
}

