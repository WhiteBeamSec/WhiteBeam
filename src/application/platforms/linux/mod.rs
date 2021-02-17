use std::fs::File;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path,
                PathBuf};
use std::process::{Command, Stdio};

pub fn start_service() {
    match Command::new("/etc/init.d/whitebeam")
            .arg("start")
            .stdout(Stdio::null())
            .spawn() {
                Ok(_p) => return,
                Err(_e) => {
                    eprintln!("WhiteBeam: Child process failed to start");
                    return;
                }
    };
}

pub fn stop_service() {
    match Command::new("/etc/init.d/whitebeam")
            .arg("stop")
            .stdout(Stdio::null())
            .spawn() {
                Ok(_p) => return,
                Err(_e) => {
                    eprintln!("WhiteBeam: Child process failed to start");
                    return;
                }
    };
}

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    #[cfg(feature = "whitelist_test")]
    let data_path: String = format!("{}/target/release/examples/", env!("PWD"));
    #[cfg(not(feature = "whitelist_test"))]
    let data_path: String = String::from("/opt/WhiteBeam/data/");
    let data_file_path = data_path + data_file;
    PathBuf::from(data_file_path)
}

pub fn path_open_secure(file_path: &Path) -> Result<File, std::io::Error> {
    Ok(std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o700)
        .open(file_path)?)
}
