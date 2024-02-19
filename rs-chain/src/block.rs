// In block.rs
use chrono::{Utc, DateTime}; //imports date time data
use sha2::{Sha256, Digest}; //import the hashing algorithm

//gets timestamp in miliseconds since 1970
pub fn initialize_timestamp() -> u64  {
    let utc: DateTime<Utc>  = Utc::now();
    utc.timestamp() as u64 //converts DateTime to UNIX timestamp (milliseconds since  1/1/1970)
}


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
    
    // Hashing  function that takes string input and returns a SHA-256 Hexadecimal string
    pub fn hash(timestamp u64, last_hash: &str, data: &str) {
        // Concatenate the timestamp lasthash, and data into a single string
        let concatenated_string = format! {"{}{}{}", timestamp, last_hash, data};
    
        //Hash the string created above using sha
        let mut hasher = Sha256::new();
        hasher.update(concatenated_string);
    
        // Get hashed value as a byte array
        let hashed_bytes = hasher.finalize();
    
        //convert the byte array into hex string
        let hashed_string= format!("{:x}", hashed_bytes);
    
        // Return the compleated hashed string
        hashed_string
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
        let timestamp = initialize_timestamp(); 
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
    let timestamp = initialize_timestamp();
    let last_hash = last_block.hash();
    let hash = hash(timestamp, lasth_hash, data); //calculates the hash of the new block's data

    Block::new(index, timestamp, data.to_string(), last_hash.to_string(), hash)
}


