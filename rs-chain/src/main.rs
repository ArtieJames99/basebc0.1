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

  // Initialize the blockchain with the genesis block
  let mut blockchain = Blockchain::initialize_blockchain();

  //Example: adds new block of transaction data
  let data = "Transaction Data for the new block";
  // Add a new block to the blockchain
  blockchain.add_block(data);

  // Print out the blockchain for demonstration
  println!("Blockchain:")
  for block in blockchain.iter_blocks() {
     println!("index: {}", block.index());
     println!("timestamp: {}", block.timestamp());
     println!("data: {}", block.data());
     println!("prev hash: {}", block.previous_hash());
     print!("hash: {}\n\n", block.hash());
     println!();
  }

}
