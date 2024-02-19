//main.rs

mod block; // Import the block.rs module
mod blockchain; //Imports the blockchain.rs module

use crate::blockchain::get_genesis_block;
use crate::blockchain::initialize_blockchain;
use block::Block; // Imports the block module
use block::Block::mine_block;


fn main() {
    // Initializes the block chain
    let blockchain = initialize_blockchain();

    //prints the string of the genesis block to console
    // Get the genesis block from the blockchain
    if let Some(genesis_block) = get_genesis_block(&blockchain) {
        // Print the details of the genesis block
        println!("Genesis Block:");
        println!("Index: {}", genesis_block.index());
        println!("Timestamp: {}", genesis_block.timestamp());
        println!("Data: {}", genesis_block.data());
        println!("Previous Hash: {}", genesis_block.previous_hash());
        println!("Hash: {}", genesis_block.hash());
    } else {
        println!("No genesis block found.");
    }

    // Create a new block and add it to the blockchain
    let new_block = Block::mine_block(last_block, &data);

    //prints information about blockchain
    for block in &blockchain {
        printin!("{:?}", block);
    }
}
