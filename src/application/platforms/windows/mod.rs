//use std::env;
use std::path::{Path,
                PathBuf};

pub fn start_service() {
    unimplemented!("WhiteBeam: Service control is not currently supported on Windows");
}

pub fn stop_service() {
    unimplemented!("WhiteBeam: Service control is not currently supported on Windows");
}

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    // TODO: Use PWD for Powershell with feature="whitelist_test"?
    // TODO: May change this when registry and environment are secured
    //PathBuf::from(env::var("ProgramFiles").unwrap_or("C:\\ProgramFiles").push_str("\\WhiteBeam\\data\\"))
    #[cfg(feature = "whitelist_test")]
    let data_path: String = format!("{}\\target\\release\\examples\\", env!("CD"));
    #[cfg(not(feature = "whitelist_test"))]
    let data_path: String = String::from("C:\\Program Files\\WhiteBeam\\data\\");
    let data_file_path = data_path + data_file;
    PathBuf::from(data_file_path)
}

pub fn path_open_secure(file_path: &Path) {
    unimplemented!("WhiteBeam: Secure file opening is not currently supported on Windows");
}

pub fn is_superuser() -> bool {
    unimplemented!("WhiteBeam: Checking user capabilities is not currently supported on Windows");
}
