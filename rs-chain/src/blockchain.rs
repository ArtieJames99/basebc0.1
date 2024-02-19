// blockchain.rs

use crate::block::Block; //Import the Block structure from the block.rs module
use crate::block::mine_block; //Import the mine_block function from the block.rs module

//Genesis block is the starting block of the block chain with no previous hash value
//Put in place as the beginning of the blockchain to provide a link for all future blocks

pub struct Blockchain {
    block: Vec<Block>,  //Vec to hold the list of blocks on the blockchain
}



//Implements the function of the blockchain
impl Blockchain {

    // Adds a new block to the end of the blockchain if it can be validly
    pub fn add_block(&mut self, block: Block) {
        self.block.push(block);
}

    pub fn initialize_blockchain() -> Self {
        // Create the genesis block
        let genesis_block = Block::genesis();
        
        // Initialize the blockchain with the genesis block
        let mut blockchain = Vec::new(){
            blockchain.push(genesis_block);
                    
        // Return the initialized blockchain explicitly
        blockchain
    }

    // Function returns the genesis block  (the first block in the blockchain)
    pub fn get_genesis_block(&self) -> Option<&Block> {
        // If blockchain not empty, return a reference of the first block
        self.block.first()
    }

    //iterate over blocks in the blockchain
    pub fn iter_blocks(&self) -> std::slice::Iter<'_, Block> {
        self.block.iter()
    }
}