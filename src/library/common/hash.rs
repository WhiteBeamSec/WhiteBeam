use sha3::{Digest, Sha3_256};
use std::{fs, io};

fn common_hash_algo() -> Sha3_256 {
    Sha3_256::new()
}

pub fn common_hash_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut hasher = common_hash_algo();
    let _n = io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.result();
    hex::encode(hash)
}
