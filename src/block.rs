use chrono::Utc;
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String, nonce: u64) -> Self {
        let timestamp = Utc::now().timestamp();
        let hash = Self::calculate_hash(index, timestamp, &data, &previous_hash, nonce);

        Self {
            index,
            data,
            previous_hash,
            timestamp,
            hash,
            nonce,
        }  
    }

    pub fn calculate_hash(index: u64, timestamp: i64, data: &String, previous_hash: &String, nonce: u64) -> String {
        let content = format!("{} {} {} {} {}", index, timestamp, data, previous_hash, nonce);

        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let res = hasher.finalize();

        format!("{:x}", res)
    }

    fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        loop {
            self.hash = Self::calculate_hash(self.index, self.timestamp, &self.data, &self.previous_hash, self.nonce);

            if self.hash.starts_with(&target) {
                break;
            }

            self.nonce += 1;
        }

        println!("{}", self.nonce);
    }
}
