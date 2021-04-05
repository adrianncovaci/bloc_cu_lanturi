use bloc_cu_lanturi_lib::*;
fn main() {
    let mut block = Block::new(0, 2341, vec![0; 64], String::from("west siiiide"));
    block.hash = block.hash();
    println!("{:?}", block.hash());
}
