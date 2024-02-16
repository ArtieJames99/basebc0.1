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
}