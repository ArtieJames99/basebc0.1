// blockchain.rs

use crate::block::Block; //Import the Block structure from the block.rs module

//Genesis block is the starting block of the block chain with no previous hash value
//Put in place as the beginning of the blockchain to provide a link for all future blocks

pub fn initialize_blockchain() -> Vec<Block> {
    // Create the genesis block
    let genesis_block = Block::genesis();
    
    // Initialize the blockchain with the genesis block
    let mut blockchain = Vec::new();
    blockchain.push(genesis_block);
    
    // Return the initialized blockchain explicitly
    blockchain
}

// Function returns the genesis block  (the first block in the blockchain)
pub fn get_genesis_block(blockchain: &[Block]) -> Option<&Block> {
    // If blockchain not empty, return a reference of the first block
    blockchain.first()
}
