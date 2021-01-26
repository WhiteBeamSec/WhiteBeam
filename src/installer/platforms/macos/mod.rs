// Load OS-specific modules

use std::path::PathBuf;

pub fn get_data_file_path(data_file: &str, release: &str) -> PathBuf {
    let data_path: String = match release {
        "test" => format!("{}/target/release/examples/", env!("PWD")),
        _ => String::from("/Applications/WhiteBeam/data/")
    };
    let data_file_path = data_path + data_file;
    PathBuf::from(data_file_path)
}

pub fn check_build_environment() {
    unimplemented!("WhiteBeam: Building on non-Linux platforms is not currently supported")
}

pub fn run_install() {
    unimplemented!("WhiteBeam: Installation on non-Linux platforms is not currently supported")
}
