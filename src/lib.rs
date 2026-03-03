use sha2::{Sha256,Digest};

#[derive(Debug)]
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

pub fn calaculate_hash_digest(data:&[u8])->[u8;32]{
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