use sha2::digest;

mod block;
mod blockchain;

fn main() {
    let genesis_block = block::Block::new(0, String::from("Genesis block"),
                                    String::from("prev block"), 0);
    println!("{:#?}", genesis_block);

    // Task 3
    let mut blockchain = blockchain::Blockchain::new(4);

    blockchain.add_block(String::from("First block after genesis"));
    blockchain.add_block(String::from("Second block"));
    blockchain.add_block(String::from("Third block"));

    blockchain.chain[1].data = String::from("hacked");

    let is_valid_res = blockchain.is_valid();
    println!("{}", is_valid_res);

    for block in &blockchain.chain {
        println!("{:#?}", block);
    }
}
