use std::path::{Path,
                PathBuf};

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    let data_path: String = String::from("/Applications/WhiteBeam/data/");
    let data_file_path = data_path + data_file;
    Path::new(&data_file_path).to_owned()
}

pub fn path_open_secure(file_path: &Path) {
    panic!("WhiteBeam: Not implemented");
}
