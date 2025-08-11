

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32
}

#[derive(Debug)]
struct Block {
    previous_hash: Vec<u8>,
    hash: Vec<u8>,
    transactions: Vec<Transaction>,
    nonce: u64,
    timestamp: DateTime<Utc>
}

impl Block {
    fn calculate_hash(&mut self) -> Vec<u8> {
        let mut new_hash = Sha256::new();

        let timestamp_of_block = self.timestamp.to_string().into_bytes();
        let prev_hash = format!("{:?}", self.previous_hash).into_bytes();
        let transactions = format!("{:?}", self.transactions).into_bytes();

        new_hash.update(timestamp_of_block);
        new_hash.update(prev_hash);
        new_hash.update(transactions);
        new_hash.update(self.nonce.to_string());

        new_hash.finalize().to_vec()
    }

    fn mine_block(&mut self, difficulty: u8) {
        let prefix = vec![0u8; difficulty as usize];

        loop {
            self.hash = self.calculate_hash();

            if self.hash.starts_with(&prefix) {
                print!("block mined");
                break;
            }
            self.nonce += 1;
        }
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: u8
}

impl Blockchain {
    fn new(diff: u8) -> Self {
        let genesis_block = Block {
            transactions: vec![],
            hash: vec![],
            previous_hash: vec![],
            timestamp: Utc::now(),
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
    println!("Hello world");

    let mut blockchain = Blockchain::new(1);

    let transaction = Transaction {
        sender: "Pratham".to_string(),
        amount: 10.0,
        receiver: "Krish".to_string()
    };

    let block = Block {
        timestamp: Utc::now(),
        transactions: vec![transaction],
        previous_hash: vec![],
        hash: vec![],
        nonce: 0
    };

    blockchain.add_new_block(block);

    println!("Blockchain: {:?}", blockchain)
}