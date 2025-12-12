mod block;
use block::Block;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();

    let genesis = Block::new(0, String::new(), timestamp, String::from("Genesis Block"));

    println!("Genesis Block: {}", genesis.to_string());
}