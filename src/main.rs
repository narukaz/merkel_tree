use merkle_tree::{MerkleTree,ProofComponent};

fn format_hash(hash: &[u8])-> String{
    hash.iter().map(|v| format!("{:02x}",v)).collect()
}

fn print_proof(proof: &[ProofComponent]){
    println!(" Proof consist of {} components",proof.len());
    for (i,component) in proof.iter().enumerate(){
        println!(" {}: Hash={} , position: = {:?}",i, format_hash(&component.hash),component.position);
    }
}
fn main() {
    let data_blocks = vec![
        b"Transaction 1".as_slice(),
        b"Transaction 2".as_slice(),
        b"Transaction 3".as_slice(),
        b"Transaction 4".as_slice(),
        b"Transaction 5".as_slice(),
    ];

    println!("Building merkle tree from {} data blocks", data_blocks.len());
    let merkle_tree = MerkleTree::from_data(&data_blocks);
    let root_hash = match &merkle_tree.root{
        Some(root) => {
            println!("Root hash:{} \n", format_hash(&root.hash));
            root.hash
        },
        None => {
            println!("Root is empty, cannot perform verification.");
            return;
        }
    };
    let leaf_to_prove_index = 2 ;
    println!("--- Test 1: Verifying a valid leaf (index {}) ---",leaf_to_prove_index );

    let leaf_hash = merkle_tree.leaves[leaf_to_prove_index].hash;
    println!("Leaf hash to prove : {}", format_hash(&leaf_hash));

    if let Some(proof) = merkle_tree.proof(leaf_to_prove_index){
        println!("Generated proof for the leaf.");
        print_proof(&proof);
        let is_valid = MerkleTree::verify(&root_hash, &leaf_hash, &proof);
        println!("\nVerification result: {}", if is_valid { "VALID" } else { "INVALID" });
        assert!(is_valid);
    } else {
        println!("Could not generate proof.");
    }
    println!("\n--- Test 2: Verifying with tampered data ---");

    let tampered_leaf_data = b"Tampered Data";
    let tampered_leaf_hash = merkle_tree::calculate_hash_digest(tampered_leaf_data);

    if let Some(proof) = merkle_tree.proof(leaf_to_prove_index){
        println!("Attacker is using the proof from leaf index {}.", leaf_to_prove_index);
        let is_valid = MerkleTree::verify(&root_hash, &tampered_leaf_hash, &proof);
        println!("\nVerification result: {}", if is_valid { "VALID" } else { "INVALID" });
        assert!(!is_valid);
        println!("Success! The tampered data was correctly detected as invalid.");
    }
}