use sodiumoxide::crypto::hash;
use std::{fs, io, io::Read, ffi::OsStr};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::os::unix::io::FromRawFd;
#[cfg(target_os = "windows")]
use std::os::windows::io::FromRawHandle;

fn common_hash_algo() -> sodiumoxide::crypto::hash::State {
    hash::State::new()
}

pub fn hash_null() -> String {
    hex::encode(vec![0; hash::DIGESTBYTES])
}

pub fn common_hash_password(input: &str) -> String {
    // TODO: Use pwhash
    hex::encode(hash::hash(input.as_bytes()))
}


pub fn common_hash_data<R: io::Read>(reader: R) -> String {
    let buf_size = 8 * 1024;
    let mut buf: Vec<u8> = Vec::with_capacity(buf_size);
    let mut hash_state = common_hash_algo();
    let mut limited_reader = reader.take(buf_size as u64);
    loop {
        match limited_reader.read_to_end(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                hash_state.update(&buf[..]);
                buf.clear();
                limited_reader = limited_reader.into_inner().take(buf_size as u64);
            }
            Err(_err) => return hash_null(),
        }
    }
    hex::encode(hash_state.finalize())
}

pub fn common_hash_fd(fd: i32) -> String {
    #[cfg(target_os = "windows")]
    unimplemented!("WhiteBeam: File handles are not currently supported");
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let file = unsafe { fs::File::from_raw_fd(fd) };
    common_hash_data(file)
}

pub fn common_hash_file(path: &OsStr) -> String {
    let file = match fs::File::open(&path) {
        Err(_why) => return hash_null(),
        Ok(file) => file
    };
    common_hash_data(file)
}
