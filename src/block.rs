use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use hex;
use serde::{Serialize, Deserialize};


pub struct Block {
    index: u64, 
    previous_hash: String,
    timestamp: u64,
    data: String,
    hash: String
}

#[derive(Serialize)]
struct BlockData {
    index: u64,
    previous_hash: String,
    timestamp: u64,
    data: String
}

impl Block {
    pub fn new(index: u64, previous_hash: String, timestamp: u64, data: String, hash: String) -> Self {
        let newHash = calculate_hash(index, previous_hash, timestamp, data, hash);
        Block { index, previous_hash, timestamp, data, newHash }
    }

    pub fn calculate_hash(&self) -> String {
        let block_data = BlockData {
            index: self.index,
            previous_hash: self.previous_hash.clone(),
            timestamp: self.timestamp,
            data: self.data.clone()
        };

        let serialized = serde_json::to_vec(&block_data).expect("Failed to serialize block data");

        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        let hash = hasher.finalize();
        
        hex::encode(hash)
    }
}