mod block;
mod blockchain;

fn main() {
    let genesis_block = block::Block::new(0, String::from("Genesis block"),
                                    String::from("prev block"));
    println!("{:#?}", genesis_block);

    // Task 3
    let mut blockchain = blockchain::Blockchain::new();

    blockchain.add_block(String::from("First block after genesis"));
    blockchain.add_block(String::from("Second block"));
    blockchain.add_block(String::from("Third block"));

    for block in &blockchain.chain {
        println!("{:#?}", block);
    }
}
