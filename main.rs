extern crate serde;
extern crate serde_json;
extern crate sha2;

use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u32,
    timestamp: u64,
    transactions: Vec<Transaction>,
    prev_hash: String,
    hash: String,
    nonce: u32,
}

impl Block {
    fn new(index: u32, timestamp: u64, transactions: Vec<Transaction>, prev_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            transactions,
            prev_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{:?}{}{}",
            self.index,
            self.timestamp,
            self.transactions,
            self.prev_hash,
            self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {} with nonce {}", self.hash, self.nonce);
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(), vec![], String::from("0"));
        self.chain.push(genesis_block);
    }

    fn add_block(&mut self, transactions: Vec<Transaction>) {
        let prev_block = &self.chain[self.chain.len() - 1];
        let new_block = Block::new(
            prev_block.index + 1,
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            transactions,
            prev_block.hash.clone(),
        );
        let mut block_to_add = new_block;
        block_to_add.mine_block(self.difficulty);
        self.chain.push(block_to_add);
    }

    fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let prev_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.prev_hash != prev_block.hash {
                return false;
            }
        }
        true
    }

    fn print_chain(&self) {
        for block in &self.chain {
            println!(
                "Block #{}: \n  Timestamp: {}\n  Transactions: {:?}\n  Previous Hash: {}\n  Current Hash: {}\n  Nonce: {}\n",
                block.index, block.timestamp, block.transactions, block.prev_hash, block.hash, block.nonce
            );
        }
    }
}

fn main() {
    let mut blockchain = Blockchain::new(4);

    blockchain.add_block(vec![
        Transaction { sender: String::from("Purvesh"), recipient: String::from("Jenish"), amount: 50 },
    ]);

    blockchain.add_block(vec![
        Transaction { sender: String::from("Jenish"), recipient: String::from("Dharmik"), amount: 30 },
    ]);

    blockchain.add_block(vec![
        Transaction { sender: String::from("Dharmik"), recipient: String::from("Uttam"), amount: 20 },
    ]);

    blockchain.print_chain();

    if blockchain.validate_chain() {
        println!("The blockchain is valid.");
    } else {
        println!("The blockchain is invalid.");
    }

    // Tampering example
    blockchain.chain[1].transactions = vec![
        Transaction { sender: String::from("Eve"), recipient: String::from("Charlie"), amount: 100 },
    ];

    if blockchain.validate_chain() {
        println!("The blockchain is valid after tampering.");
    } else {
        println!("The blockchain is invalid after tampering.");
    }
}
