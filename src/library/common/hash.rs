use sha3::{Digest, Sha3_256};
use std::{fs, io, ffi::OsStr};
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

pub fn common_hash_fd(fd: i32) -> String {
    #[cfg(target_os = "windows")]
    unimplemented!("WhiteBeam: File handles are not currently supported");
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let mut file = unsafe { fs::File::from_raw_fd(fd) };
    let mut hasher = common_hash_algo();
    let _n = match io::copy(&mut file, &mut hasher) {
        Err(_why) => return hash_null(),
        Ok(cnt) => cnt
    };
    let hash = hasher.result();
    hex::encode(hash)
}

pub fn common_hash_file(path: &OsStr) -> String {
    let mut file = match fs::File::open(&path) {
        Err(_why) => return hash_null(),
        Ok(file) => file
    };
    let mut hasher = common_hash_algo();
    let _n = match io::copy(&mut file, &mut hasher) {
        Err(_why) => return hash_null(),
        Ok(cnt) => cnt
    };
    let hash = hasher.result();
    hex::encode(hash)
}
