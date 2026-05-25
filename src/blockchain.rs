use std::fmt::Debug;

use crate::{block::Block, transaction::Transaction};

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut chain = Vec::<Block>::new();

        let genesis_block = Block::new(0, String::from("Genesis block"), String::from("0"), 0);

        chain.push(genesis_block);

        Self {
            chain,
            difficulty,
            pending_transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn add_block(&mut self) {
        let last_block = self.chain.last().unwrap();

        /*let mut block_data = String::new();
        for transaction in &self.pending_transactions {
            block_data.push_str(&transaction.to_string());
            block_data.push('\n');
        }*/

        let data = serde_json::to_string(&self.pending_transactions)
                            .expect("Failed to serialize transactions");

        let new_block = Block::new(
            last_block.index + 1, 
            data, 
            last_block.hash.clone(), 
            0);

        self.chain.push(new_block);
        self.pending_transactions.clear();
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let recalculated_hash = Block::calculate_hash(
                self.chain[i].index,
                self.chain[i].timestamp,
                &self.chain[i].data,
                &self.chain[i].previous_hash,
                self.chain[i].nonce);

            if recalculated_hash != self.chain[i].hash {
                return false;
            }

            if self.chain[i - 1].hash != self.chain[i].previous_hash {
                return false;
            }
        }

        true
    }
}