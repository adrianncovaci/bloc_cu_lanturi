use bloc_cu_lanturi_lib::*;
fn main() {
    let mut block = Block::new(
        0,
        2341,
        vec![0; 64],
        String::from("west siiiide"),
        0x0000ffffffffffffffffffffffffffff,
    );
    block.hash = block.hash();
    println!("{:?} - nonce: {}", block.hash, block.nonce);

    block.calculate_hash();
    println!("{:?} - nonce: {}", block.hash, block.nonce);
}
