use sha3::{Digest, Sha3_256};
use std::{fs, io};

fn common_hash_algo() -> Sha3_256 {
    Sha3_256::new()
}

pub fn common_hash_file(path: &str) -> String {
    let mut file = match fs::File::open(&path) {
        Err(why) => return "0".repeat(64),
        Ok(file) => file
    };
    let mut hasher = common_hash_algo();
    let _n = match io::copy(&mut file, &mut hasher) {
        Err(why) => return "0".repeat(64),
        Ok(cnt) => cnt
    };
    let hash = hasher.result();
    hex::encode(hash)
}
