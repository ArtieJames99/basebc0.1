//main.rs

mod block; // Import the block.rs module
mod blockchain; //Imports the blockchain.rs module

use block::Block; // Imports the block module
use blockchain::Blockchain;

fn main() {
    // Initializes the block chain
   // let blockchain = initialize_blockchain();

    //prints the string of the genesis block to console
    // Get the genesis block from the blockchain
   // if let Some(genesis_block) = get_genesis_block(&blockchain) {
        // Print the details of the genesis block
   //     println!("Genesis Block:");
   //     println!("Index: {}", genesis_block.index());
   //     println!("Timestamp: {}", genesis_block.timestamp());
   //     println!("Data: {}", genesis_block.data());
   //     println!("Previous Hash: {}", genesis_block.previous_hash());
  //      println!("Hash: {}", genesis_block.hash());
  //  } else {
  //      println!("No genesis block found.");
  //  }

  let mut blockchain = Blockchain::new();
  blockchain.add_block("Transaction data for the new block")

}
