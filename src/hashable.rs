use blake2::{Blake2b, Digest};
pub trait Hashable {
    fn to_bytes(&self) -> Vec<u8>;
    fn hash(&self) -> Vec<u8> {
        let mut hasher = Blake2b::new();
        hasher.input(self.to_bytes());
        hasher.result().to_vec()
    }
}
