//block.rs

//defines the structure of the block
pub struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    //constructor funcion to make a new instance
    pub fn new(index: u32, timestamp: u64, data: String, previous_hash:String, hash: String) -> Self{
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    pub fn genesis() -> Self {
        //defines genesis properties
        let index = 0;
        let timestamp = "Genisis time".to_string();
        let data = "Genesis Block - Crated by <AJ Jex>".to_string();
        let previous_hash = String::from("0"repeat(64)); //Assuming a 64-character hash
        let hash = "f1r57-h45h".to_string(); //First hash in the blockchain
        
        Block::new(index,timestamp,data,previous_hash,hash)
    }
}