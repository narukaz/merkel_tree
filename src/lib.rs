use sha2::{Sha256,Digest};

#[derive(Debug,Clone)]
pub struct MerkleNode{
    pub hash: [u8;32],
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

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
pub struct MerkelTree{
    pub root: Option<MerkleNode>
}
impl MerkelTree{
    pub fn create_internal_node(left_child: Box<MerkleNode>, right_child: Box<MerkleNode>)-> MerkleNode{
        let mut combined_hash_data = [0u8;64];
        combined_hash_data[..32].copy_from_slice(&left_child.hash);
        combined_hash_data[32..].copy_from_slice(&right_child.hash);
        let new_hash = calculate_hash_digest(&combined_hash_data);
         MerkleNode::new(new_hash,Some(left_child),Some(right_child))

    }
    pub fn from_data(data: &[&[u8]])->Self{
        if data.is_empty(){
            return MerkelTree::new()
        };


        let mut  nodes : Vec<MerkleNode> =data.iter().map(|d|{
            let hash = calculate_hash_digest(d);
            MerkleNode::new(hash,None,None)
        }).collect();

        while nodes.len() > 1 {
            if nodes.len() % 2 != 0 {
                nodes.push(nodes.last().unwrap().clone());
            }

           let next_level_nodes : Vec<MerkleNode> = nodes.chunks(2).map(|pair|{
               let left = pair[0].clone();
               let right = pair[1].clone();
               MerkelTree::create_internal_node(Box::new(left), Box::new(right))
                    }).collect::<Vec<MerkleNode>>();

            nodes = next_level_nodes;


        }





   MerkelTree{
       root:nodes.pop()
   }



    }
    pub fn new()-> Self{
        MerkelTree{
            root:None
        }
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
    fn test_hash_calculation(){
        let input = "hello world!";
        let hash_result = calculate_hash_updates(input.as_bytes());
        let expected_hash: [u8; 32] = [187, 9, 229, 189, 160, 199, 98, 210, 186, 199, 249, 13, 117, 139, 91, 34, 99, 250, 1, 204, 188, 84, 42, 181, 227, 223, 22, 59, 224, 142, 108, 169];
        assert_ne!(hash_result,expected_hash, "The calculated hash do not match the expected hash");
    }
    #[test]
    fn test_hash_match(){
        let input = "hello world!";
        let hash_result = calculate_hash_updates(input.as_bytes());
        let expected_hash:[u8;32] =  [117, 9, 229, 189, 160, 199, 98, 210, 186, 199, 249, 13, 117, 139, 91, 34, 99, 250, 1, 204, 188, 84, 42, 181, 227, 223, 22, 59, 224, 142, 108, 169];
        assert_eq!(hash_result,expected_hash, "The calculated hash matched");
    }
    #[test]
    fn test_empty_input(){
        let input = "";
        let hash_result = calculate_hash_updates(input.as_bytes());
        println!("hash empty{:?}",hash_result);
        assert_eq!(hash_result,[
            227, 176, 196, 66, 152, 252, 28, 20,
            154, 251, 244, 200, 153, 111, 185, 36,
            39, 174, 65, 228, 100, 155, 147, 76,
            164, 149, 153, 27, 120, 82, 184, 85
        ], "Hash of empty string is incorrect");
    }
}