use sha2::{Sha256, Digest};
use hex;
use serde::{Serialize};


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
    pub fn new(index: u64, previous_hash: String, timestamp: u64, data: String) -> Self {
        let mut block = Block { index, previous_hash, timestamp, data, hash: String::new()};
        block.hash = block.calculate_hash();
        return block;
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