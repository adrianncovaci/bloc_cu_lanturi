use crate::hashable::*;
use crate::{check_difficulty, get_timestamp, BlockHash};

#[derive(Debug)]
pub struct Block {
    timestamp: u64,
    prev_hash: BlockHash,
    pub hash: BlockHash,
    payload: String,
    pub nonce: u128,
    id: u128,
    difficulty: u128,
}

impl Block {
    pub fn new(
        id: u128,
        nonce: u128,
        prev_hash: BlockHash,
        payload: String,
        difficulty: u128,
    ) -> Self {
        Self {
            id,
            nonce,
            prev_hash,
            hash: vec![0; 64],
            payload,
            difficulty,
            timestamp: get_timestamp(),
        }
    }
    pub fn calculate_hash(&mut self) {
        for nonce_iter in 0..u128::max_value() {
            self.nonce = nonce_iter;
            self.hash = self.hash();
            if check_difficulty(&self.hash, &self.difficulty) {
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&self.id.to_be_bytes());
        bytes.extend(&self.nonce.to_be_bytes());
        bytes.extend(&self.prev_hash);
        bytes.extend(&self.timestamp.to_be_bytes());
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&self.difficulty.to_be_bytes());
        bytes
    }
}
