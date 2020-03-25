// Load OS-specific modules

//use std::env;
use std::{path::Path,
          path::PathBuf};

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    // TODO: Change this when registry and environment are secured
    //Path::new(env::var("ProgramFiles").unwrap_or("C:\\ProgramFiles").push_str("\\WhiteBeam\\data\\"))
    let data_path: String = String::from("C:\\Program Files\\WhiteBeam\\data\\");
    let data_file_path = data_path + data_file;
    Path::new(&data_file_path).to_owned()
}

pub fn check_build_environment() {
    unimplemented!("WhiteBeam: Building on non-Linux platforms is not currently supported")
}

pub fn run_install() {
    unimplemented!("WhiteBeam: Installation on non-Linux platforms is not currently supported")
}
