use std::convert::TryInto;
pub type BlockHash = Vec<u8>;

pub fn get_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn check_difficulty(hash: &BlockHash, diff: &u128) -> bool {
    diff > &u128::from_be_bytes(hash[0..16].try_into().expect("incorrect length"))
}

mod hashable;
mod models;
pub use crate::hashable::Hashable;
pub use crate::models::block::Block;
