// In block.rs
use chrono::{Utc, DateTime}; //imports date time data

//gets timestamp in miliseconds since 1970
pub const TIMESTAMP:u64 = {
    let utc: DateTime<Utc>  = Utc::now();
    utc.timestamp_millis() as u64
};


pub struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    // Constructor function to make a new instance
    pub fn new(index: u32, timestamp: u64, data: String, previous_hash: String, hash: String) -> Self {
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
    
    // Below sets the presets for the genesis block
    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn data(&self) -> &str {
        &self.data
    }

    pub fn previous_hash(&self) -> &str {
        &self.previous_hash
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn genesis() -> Self {
        // Define genesis properties
        let index = 0;
        let timestamp = 1234567890; // Example timestamp (replace with your actual timestamp)
        let data = String::from("Genesis Block - Created by AJ Jex");
        let previous_hash = String::from("0").repeat(64); // Assuming a 64-character hash
        let hash = String::from("f1r57-h45h"); // First hash in the blockchain

        Block::new(index, timestamp, data, previous_hash, hash)
    }
    //genesis block ends here
}

//static mineblock function

pub fn mine_block(last_block: &Block, data:  &str) -> Block {
    let index = 1;
    let timestamp = TIMESTAMP;
    let last_hash = last_block.hash();
    let hash = String::from("test-hash");

    Block::new(index, timestamp, data.to_string(), last_hash.to_string(), hash)
}