use crate::hashable::*;
use crate::{get_timestamp, BlockHash};

#[derive(Debug)]
pub struct Block {
    timestamp: u64,
    prev_hash: BlockHash,
    pub hash: BlockHash,
    payload: String,
    nonce: u128,
    id: u128,
}

impl Block {
    pub fn new(id: u128, nonce: u128, prev_hash: BlockHash, payload: String) -> Self {
        Self {
            id,
            nonce,
            prev_hash,
            hash: vec![0; 64],
            payload,
            timestamp: get_timestamp(),
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
        bytes
    }
}
