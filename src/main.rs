use std::fmt::format;
use merkle_tree::MerkelTree;

fn main() {
    let data_blocks = vec![
        b"Transaction 1".as_slice(),
        b"Transaction 2".as_slice(),
        b"Transaction 3".as_slice(),
        b"Transaction 4".as_slice(),
        b"Transaction 5".as_slice(),
    ];
    println!("Building Merkel Tree for {} data blocks...",data_blocks.len());
    let merkle_tree = MerkelTree::from_data(&data_blocks);

    if let Some(root) = merkle_tree.root{
        let root_hash_hex = root.hash.iter()
            .map(|b| format!("{:02x}",b))
            .collect::<String>();
        println!("Merkle Tree construction complete!");
        println!("Root Hash: {}", root_hash_hex);
    }
    else {
        println!("The Merkle tree is empty.");
    }

}