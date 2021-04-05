pub type BlockHash = Vec<u8>;

pub fn get_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

mod hashable;
mod models;
pub use crate::hashable::Hashable;
pub use crate::models::block::Block;
