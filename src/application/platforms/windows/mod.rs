//use std::env;
use std::path::{Path,
                PathBuf};

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    // TODO: Change this when registry and environment are secured
    //Path::new(env::var("ProgramFiles").unwrap().push_str("\\WhiteBeam\\data\\"))
    let data_path: String = String::from("C:\\Program Files\\WhiteBeam\\data\\");
    let data_file_path = data_path + data_file;
    Path::new(&data_file_path).to_owned()
}

pub fn path_open_secure(file_path: &Path) {
    panic!("WhiteBeam: Not implemented");
}
