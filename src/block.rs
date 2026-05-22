use chrono::Utc;
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let hash = Self::calculate_hash(index, timestamp, &data, &previous_hash);

        Self {
            index,
            data,
            previous_hash,
            timestamp,
            hash,
        }  
    }

    pub fn calculate_hash(index: u64, timestamp: i64, data: &String, previous_hash: &String) -> String {
        let content = format!("{} {} {} {}", index, timestamp, data, previous_hash);

        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let res = hasher.finalize();

        format!("{:x}", res)
    }
}
