use std::path::{Path,
                PathBuf};

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    let data_path: String = "/opt/WhiteBeam/data/".to_owned();
    let data_file_path = data_path + data_file;
    Path::new(&data_file_path).to_owned()
}
