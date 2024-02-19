//main.rs

mod block; // Import the block.rs module
mod blockchain; //Imports the blockchain.rs module

use block::Block; // Imports the block module
use blockchain::Blockchain;

fn main() {
  // Initialize the blockchain with the genesis block
  let mut blockchain = Blockchain::initialize_blockchain();

  //Example: adds new block of transaction data
  let data = "Transaction Data for the new block";
  // Add a new block to the blockchain
  blockchain.add_block(data);

  // Print out the blockchain for demonstration
  println!("Blockchain");
  for block in blockchain.iter_blocks() {
     println!("index: {}", block.index());
     println!("timestamp: {}", block.timestamp());
     println!("data: {}", block.data());
     println!("prev hash: {}", block.previous_hash());
     print!("hash: {}\n\n", block.hash());
     println!();
  }
}
