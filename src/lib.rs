use sha2::{Sha256,Digest};

#[derive(Debug,Clone)]
pub struct MerkleNode{
    pub hash: [u8;32],
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

#[derive(Debug,Clone)]
pub enum Position{
    Left,
    Right,
}

#[derive(Debug,Clone)]
pub struct ProofComponent{
    pub hash: [u8;32],
    pub position: Position

}
pub type MerkleProof = Vec<ProofComponent>;

impl MerkleNode {
    pub fn new(hash:[u8;32],left:Option<Box<MerkleNode>>, right: Option<Box<MerkleNode>>)-> Self{
                MerkleNode{
                    hash,
                    left,
                    right
                }
    }

}

#[derive(Debug)]
pub struct MerkleTree{
    pub root: Option<MerkleNode>,
    pub leaves: Vec<MerkleNode>
}
impl MerkleTree{
    pub fn create_internal_node(left_child: Box<MerkleNode>, right_child: Box<MerkleNode>)-> MerkleNode{
        let mut combined_hash_data = [0u8;64];
        combined_hash_data[..32].copy_from_slice(&left_child.hash);
        combined_hash_data[32..].copy_from_slice(&right_child.hash);
        let new_hash = calculate_hash_digest(&combined_hash_data);
         MerkleNode::new(new_hash,Some(left_child),Some(right_child))

    }
    pub fn verify(root_hash:&[u8;32], leaf_hash:&[u8;32], proof: &MerkleProof) -> bool {
        let mut computed_hash = leaf_hash.clone();
        for component in proof.iter(){
            match component.position {
                Position::Right =>{
                    let mut combined_data = Vec::new();
                    combined_data.extend_from_slice(&computed_hash);
                    combined_data.extend_from_slice(&component.hash);
                    computed_hash = calculate_hash_digest(&combined_data);
                },
                Position::Left => {
                    let mut combined_data = Vec::new();
                    combined_data.extend_from_slice(&component.hash);
                    combined_data.extend_from_slice(&computed_hash);

                    computed_hash = calculate_hash_digest(&combined_data);
                }
            }
        }
        computed_hash == *root_hash
    }
    pub fn from_data(data: &[&[u8]])->Self{
        if data.is_empty(){
            return MerkleTree::new()
        };
        let leaves: Vec<MerkleNode>= data.iter().map(|d|{
            let hash = calculate_hash_digest(d);
            MerkleNode::new(hash,None,None)
        }).collect();
        let mut nodes = leaves.clone();

        // let mut  nodes : Vec<MerkleNode> =data.iter().map(|d|{
        //     let hash = calculate_hash_digest(d);
        //     MerkleNode::new(hash,None,None)
        // }).collect();

        while nodes.len() > 1 {
            if nodes.len() % 2 != 0 {
                nodes.push(nodes.last().unwrap().clone());
            }

           let next_level_nodes : Vec<MerkleNode> = nodes.chunks(2).map(|pair|{
               let left = pair[0].clone();
               let right = pair[1].clone();
               MerkleTree::create_internal_node(Box::new(left), Box::new(right))
                    }).collect::<Vec<MerkleNode>>();

            nodes = next_level_nodes;


        }
        MerkleTree{
       root:nodes.pop(),
       leaves
   }
    }
    pub fn new()-> Self{
        MerkleTree{
            root:None,
            leaves:Vec::new()
        }
    }

    pub fn proof(&self, leaf_index:usize)-> Option<MerkleProof>{
        if leaf_index >= self.leaves.len(){
            return None //out of bound call
        }
        let mut proof_components = Vec::new();
        let mut current_index = leaf_index;
        let mut current_level_nodes = self.leaves.clone();
        while current_level_nodes.len() > 1 {
            if current_level_nodes.len() %2 != 0 {
                current_level_nodes.push(current_level_nodes.last().unwrap().clone());
            }
            let sibling_index = current_index ^ 1;

            if let Some(sibling_node) = current_level_nodes.get(sibling_index){
                let position = if current_index % 2 == 0 {
                        Position::Right
                }else{
                   Position::Left
                };
                proof_components.push(ProofComponent{hash:sibling_node.hash,position});
            }

            current_index /= 2;
            current_level_nodes = current_level_nodes.chunks(2).map(|pair|{
                let left = pair[0].clone();
                let right = pair[1].clone();
                MerkleTree::create_internal_node(Box::new(left),Box::new(right))
            }).collect();
        }
        Some(proof_components)
    }
}
pub fn calculate_hash_updates(data: &[u8])->[u8;32]{
let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.into()
}

pub fn calculate_hash_digest(data:&[u8])->[u8;32]{
    let mut result = Sha256::digest(data);
    result.into()
}





#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash_calculation() {
        let input = "hello world!";
        let hash_result = calculate_hash_updates(input.as_bytes());
        let expected_hash: [u8; 32] = [187, 9, 229, 189, 160, 199, 98, 210, 186, 199, 249, 13, 117, 139, 91, 34, 99, 250, 1, 204, 188, 84, 42, 181, 227, 223, 22, 59, 224, 142, 108, 169];
        assert_ne!(hash_result, expected_hash, "The calculated hash do not match the expected hash");
    }
    #[test]
    fn test_hash_match() {
        let input = "hello world!";
        let hash_result = calculate_hash_updates(input.as_bytes());
        let expected_hash: [u8; 32] = [117, 9, 229, 189, 160, 199, 98, 210, 186, 199, 249, 13, 117, 139, 91, 34, 99, 250, 1, 204, 188, 84, 42, 181, 227, 223, 22, 59, 224, 142, 108, 169];
        assert_eq!(hash_result, expected_hash, "The calculated hash matched");
    }
    #[test]
    fn test_empty_input() {
        let input = "";
        let hash_result = calculate_hash_updates(input.as_bytes());
        println!("hash empty{:?}", hash_result);
        assert_eq!(hash_result, [
            227, 176, 196, 66, 152, 252, 28, 20,
            154, 251, 244, 200, 153, 111, 185, 36,
            39, 174, 65, 228, 100, 155, 147, 76,
            164, 149, 153, 27, 120, 82, 184, 85
        ], "Hash of empty string is incorrect");
    }

    #[test]
    fn test_empty_tree() {
        let data: Vec<&[u8]> = Vec::new();
        let tree = MerkleTree::from_data(&data);
        assert!(tree.root.is_none(), "An empty tree should have no roots");
        assert!(tree.leaves.is_empty(), "An empty tree should have no leaves")
    }

    #[test]
    fn test_single_leaf_tree() {
        let data = vec![b"one".as_slice()];
        let tree = MerkleTree::from_data(&data);
        let root_hash = tree.root.as_ref().unwrap().hash;
        let leaf_hash = tree.leaves.first().unwrap().hash;
        assert_eq!(root_hash, leaf_hash, "Root of a single-leaf tree should be the leaf's hash.");
        let proof = tree.proof(0).unwrap();
        assert!(proof.is_empty(), "proof of single leaf should be empty");
        let is_valid = MerkleTree::verify(&root_hash, &leaf_hash, &proof);
        assert!(is_valid, "Verification of a single leaf with an empty proof should be valid.");
    }

    #[test]
    fn test_even_leaf_tree() {
        let data: &[&[u8]] = &[b"1", b"2", b"3", b"4"];
        let tree = MerkleTree::from_data(&data);
        let root_hash = tree.root.as_ref().unwrap().hash;
        let h1 = calculate_hash_digest(b"1");
        let h2 = calculate_hash_digest(b"2");
        let h3 = calculate_hash_digest(b"3");
        let h4 = calculate_hash_digest(b"4");
        let h12 = calculate_hash_digest(&[h1.as_slice(), h2.as_slice()].concat());
        let h34 = calculate_hash_digest(&[h3.as_slice(), h4.as_slice()].concat());
        let expected_root = calculate_hash_digest(&[h12.as_slice(), h34.as_slice()].concat());
        let leaf_to_prove = &h3;
        let proof = tree.proof(2).unwrap();
        let is_valid = MerkleTree::verify(&root_hash, leaf_to_prove, &proof);
        assert!(is_valid, "Proof verification failed for an even-leaf tree.");
    }



    #[test]
    fn test_odd_leaves_tree() {
        let data:&[&[u8]] = &[b"1", b"2", b"3"];
        let tree = MerkleTree::from_data(&data);
        let root_hash = tree.root.as_ref().unwrap().hash;
        let h1 = calculate_hash_digest(b"1");
        let h2 = calculate_hash_digest(b"2");
        let h3 = calculate_hash_digest(b"3");
        let h12 = calculate_hash_digest(&[h1.as_slice(), h2.as_slice()].concat());
        let h33 = calculate_hash_digest(&[h3.as_slice(), h3.as_slice()].concat());
        let expected_root = calculate_hash_digest(&[h12.as_slice(), h33.as_slice()].concat());
        assert_eq!(root_hash, expected_root, "Merkle root for odd leaves is incorrect.");
        let leaf_to_prove = &h3;
        let proof = tree.proof(2).unwrap();
        let is_valid = MerkleTree::verify(&root_hash, leaf_to_prove, &proof);
        assert!(is_valid, "Proof verification failed for the last leaf of an odd-leaf tree.");
    }


    #[test]
    fn test_tampered_data_fails_verification() {
        // Create a tree with some data.
        let data : &[&[u8]]=  &[b"good", b"data", b"here", b"safe"];
        let tree = MerkleTree::from_data(&data);
        let root_hash = tree.root.as_ref().unwrap().hash;

        let valid_proof = tree.proof(1).unwrap();

        let tampered_leaf_hash = calculate_hash_digest(b"evil attacker data");

        let is_valid = MerkleTree::verify(&root_hash, &tampered_leaf_hash, &valid_proof);


        assert!(!is_valid, "Verification should fail for tampered leaf data.");
    }


    #[test]
    fn test_tampered_proof_fails_verification() {
        // Create a tree.
        let data : &[&[u8]]= &[b"good", b"data", b"here", b"safe"];
        let tree = MerkleTree::from_data(&data);
        let root_hash = tree.root.as_ref().unwrap().hash;
        let real_leaf_hash = tree.leaves[1].hash;
        let mut tampered_proof = tree.proof(1).unwrap();
        if !tampered_proof.is_empty() {
            tampered_proof[0].hash = calculate_hash_digest(b"garbage");
        } else {

            return;
        }
        let is_valid = MerkleTree::verify(&root_hash, &real_leaf_hash, &tampered_proof);
        assert!(!is_valid, "Verification should fail for a tampered proof.");
    }
}

