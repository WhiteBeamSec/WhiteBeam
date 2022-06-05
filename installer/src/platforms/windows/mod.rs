// Load OS-specific modules

//use std::env;
use std::path::PathBuf;

pub fn get_data_file_path(data_file: &str, release: &str) -> PathBuf {
    // TODO: Use PWD for Powershell with feature="whitelist_test"?
    // TODO: May change this when registry and environment are secured
    //PathBuf::from(env::var("ProgramFiles").unwrap_or("C:\\ProgramFiles").push_str("\\WhiteBeam\\data\\"))
    let data_path: String = match release {
        "test" => format!("{}\\target\\release\\examples\\data\\", env!("CD")),
        _ => String::from("C:\\Program Files\\WhiteBeam\\data\\")
    };
    let data_file_path = data_path + data_file;
    PathBuf::from(data_file_path)
}

pub fn get_realtime_file_path(realtime_file: &str, release: &str) -> PathBuf {
    // TODO: Use PWD for Powershell with feature="whitelist_test"?
    // TODO: May change this when registry and environment are secured
    //PathBuf::from(env::var("ProgramFiles").unwrap_or("C:\\ProgramFiles").push_str("\\WhiteBeam\\realtime\\"))
    let realtime_path: String = match release {
        "test" => format!("{}\\target\\release\\examples\\realtime\\", env!("CD")),
        _ => String::from("C:\\Program Files\\WhiteBeam\\realtime\\")
    };
    let realtime_file_path = realtime_path + realtime_file;
    PathBuf::from(realtime_file_path)
}

pub fn check_build_environment() {
    unimplemented!("WhiteBeam: Building on non-Linux platforms is not currently supported")
}

pub fn run_install() {
    unimplemented!("WhiteBeam: Installation on non-Linux platforms is not currently supported")
}

pub fn run_package() {
    unimplemented!("WhiteBeam: Packaging on non-Linux platforms is not currently supported")
}