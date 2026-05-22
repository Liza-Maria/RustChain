use crate::block::Block;


pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut chain = Vec::<Block>::new();

        let genesis_block = Block::new(0, String::from("Genesis block"), String::from('0'), 0);

        chain.push(genesis_block);

        Blockchain { chain }
    }

    pub fn add_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap();

        let new_block = Block::new(last_block.index + 1, data, last_block.hash.clone(), 0);

        self.chain.push(new_block);
    }
}