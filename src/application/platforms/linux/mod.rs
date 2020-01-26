use std::fs::File;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path,
                PathBuf};

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    let data_path: String = String::from("/opt/WhiteBeam/data/");
    let data_file_path = data_path + data_file;
    Path::new(&data_file_path).to_owned()
}

pub fn path_open_secure(file_path: &Path) -> File {
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o700)
        .open(file_path)
        .expect(&format!("WhiteBeam: Could not securely open path {}", file_path.to_string_lossy()))
}
