use std::vec;

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
    transactions: Vec<Transaction>,
    nonce: u64
}

impl Block {
    fn calculate_hash(&mut self) -> Vec<u8> {
        let mut new_hash = Sha256::new();

        let timestamp = self.timestamp.to_string().into_bytes();
        let previous_hash = format!("{:?}", self.previous_hash).into_bytes();
        let transaction_bytes = format!("{:?}", self.transactions).into_bytes();

        new_hash.update(timestamp);
        new_hash.update(transaction_bytes);
        new_hash.update(previous_hash);
        new_hash.update(self.nonce.to_string());

        new_hash.finalize().to_vec()
    }

    fn mine_block(&mut self, difficulty: u32) {
        let prefix = vec![0u8; difficulty as usize];

        loop {
            self.hash = self.calculate_hash();
            if self.hash.starts_with(&prefix) {
                println!("Block mined {:?}", self);
                break;
            }
            self.nonce += 1;
        }
    }
}
// struct for blockchain
#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: u32
}

impl Blockchain {
    // genesis block 
    fn new(diff: u32) -> Self {

        let genesis_block = Block {
            timestamp: Utc::now(),
            previous_hash: vec![0,32],
            hash: vec![],
            transactions: vec![],
            nonce: 0
        };

        let mut blockchain = Blockchain {
            chain: vec![genesis_block],
            difficulty: diff
        };

        blockchain.chain[0].mine_block(diff);

        blockchain
    }

    fn add_new_block(&mut self, mut block: Block) {
        let prev_hash = self.chain.last().unwrap().hash.clone();

        block.previous_hash = prev_hash;

        block.mine_block(self.difficulty);

        self.chain.push(block);
    }




}

fn main() {
    println!("Hello, world!");

    let mut blockchain = Blockchain::new(0);

    let transaction = Transaction {
        sender: "Pratham".to_string(),
        amount: 10.00,
        receiver: "Krish".to_string()
    };

    let block = Block {
        timestamp: Utc::now(),
        previous_hash: vec![],
        hash: vec![],
        transactions: vec![transaction],
        nonce: 0
    };

    blockchain.add_new_block(block);

    println!("{:?}", blockchain)
}

