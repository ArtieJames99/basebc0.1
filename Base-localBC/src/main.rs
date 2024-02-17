//main.rs

mod block; // Import the block.rs module
mod blockchain; //Imports the blockchain.rs module


use block::Block; // Imports the block module

fn main() {
    // Initializes the block chain
    let blockchain = initialize_blockchain();

    //prints the string of the genesis block to console
    println!("{}", blockchain.get_genesis_block());
}