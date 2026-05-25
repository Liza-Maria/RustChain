mod block;
mod blockchain;
mod transaction;

fn main() {
    let genesis_block = block::Block::new(0, String::from("Genesis block"),
                                    String::from("prev block"), 0);
    println!("{:#?}", genesis_block);

    // Task 3
    let mut blockchain = blockchain::Blockchain::new(4);

    blockchain.add_transaction(transaction::Transaction::new(String::from("Alice"), String::from("Bob"), 26.0));
    blockchain.add_transaction(transaction::Transaction::new(String::from("Bob"), String::from("Charlie"), 48.5));
    
    blockchain.add_block();
    blockchain.add_block();
    blockchain.add_block();

    // Part 1 - Task 5 
    let is_valid_res = blockchain.is_valid();
    println!("{}", is_valid_res);


    for block in &blockchain.chain {
        println!("{:#?}", block);
    }
}
