// Load OS-specific modules
// TODO: DYLD_INSERT_LIBRARIES globally

use std::path::PathBuf;

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    #[cfg(feature = "whitelist_test")]
    let data_path: String = format!("{}/target/release/examples/", env!("PWD"));
    #[cfg(not(feature = "whitelist_test"))]
    let data_path: String = String::from("/Applications/WhiteBeam/data/");
    let data_file_path = data_path + data_file;
    PathBuf::from(data_file_path)
}
