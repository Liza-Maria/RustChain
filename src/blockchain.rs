use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Blockchain {
        let mut chain = Vec::<Block>::new();

        let genesis_block = Block::new(0, String::from("Genesis block"), String::from('0'), 0);

        chain.push(genesis_block);

        Blockchain {
            chain,
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap();

        let new_block = Block::new(last_block.index + 1, data, last_block.hash.clone(), 0);

        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let recalculated_hash = Block::calculate_hash(
                self.chain[i].index,
                self.chain[i].timestamp,
                &self.chain[i].data,
                &self.chain[i].previous_hash,
                self.chain[i].nonce);

            if recalculated_hash != self.chain[i].hash {
                return false;
            }

            if self.chain[i - 1].hash != self.chain[i].previous_hash {
                return false;
            }
        }

        true
    }
}