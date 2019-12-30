use sha3::{Digest, Sha3_256};
use std::{fs, io};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::os::unix::io::FromRawFd;
#[cfg(target_os = "windows")]
use std::os::windows::io::FromRawHandle;

fn common_hash_algo() -> Sha3_256 {
    Sha3_256::new()
}

pub fn hash_null() -> String {
    "0".repeat(64)
}

pub fn common_hash_string(input: &str) -> String {
    let mut hasher = common_hash_algo();
    hasher.input(input);
    format!("{:x}", hasher.result())
}

pub fn common_hash_file(path: &str) -> String {
    let mut file = if path.starts_with("fd://") {
        let fdno = match path.replace("fd://","").parse::<i32>() {
            Err(_why) => return hash_null(),
            Ok(fd) => fd
        };
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        unsafe { fs::File::from_raw_fd(fdno) }
        #[cfg(target_os = "windows")]
        return hash_null();
    } else {
        match fs::File::open(&path) {
            Err(_why) => return hash_null(),
            Ok(file) => file
        }
    };
    let mut hasher = common_hash_algo();
    let _n = match io::copy(&mut file, &mut hasher) {
        Err(_why) => return hash_null(),
        Ok(cnt) => cnt
    };
    let hash = hasher.result();
    hex::encode(hash)
}
