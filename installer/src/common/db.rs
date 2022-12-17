#[cfg(target_os = "windows")]
use crate::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::platforms::macos as platform;
use std::{error::Error,
          path::PathBuf,
          process::Command,
          process::Stdio};

fn db_init() -> Result<(), Box<dyn Error>> {
    db_load("Schema")?;
    db_load("Default")?;
    let bin_target_path: PathBuf = PathBuf::from(format!("{}/target/release/whitebeam", env!("PWD")));
    let _status = Command::new(&bin_target_path).args(&["--setting", "SystemArchitecture", std::env::consts::ARCH]).stdout(Stdio::null()).stderr(Stdio::null()).status()?;
    if PathBuf::from("/lib64/libc.so.6").exists() {
        let _status_lib_path = Command::new(&bin_target_path).args(&["--setting", "SystemLibraryPath", "/lib64/"]).stdout(Stdio::null()).stderr(Stdio::null()).status()?;
    } else {
        let _status_lib_path = Command::new(&bin_target_path).args(&["--setting", "SystemLibraryPath", &format!("/lib/{}-linux-gnu/", std::env::consts::ARCH)]).stdout(Stdio::null()).stderr(Stdio::null()).status()?;
    }
    Ok(())
}

pub fn db_optionally_init(release: &str) -> Result<(), Box<dyn Error>> {
    let is_test: bool = release == "test";
    // Ensure data and realtime directories exist
    let data_dir: PathBuf = platform::get_data_file_path("", release);
    if !((&data_dir).exists()) {
        std::fs::create_dir_all(&data_dir)?;
    }
    let realtime_dir: PathBuf = platform::get_realtime_file_path("", release);
    if !((&realtime_dir).exists()) {
        std::fs::create_dir_all(&realtime_dir)?;
    }
    let db_path: PathBuf = platform::get_data_file_path("database.sqlite", release);
    let realtime_db_path: PathBuf = platform::get_realtime_file_path("database.sqlite", release);
    // Always reinitialize database for testing
    if is_test && (&db_path).exists() {
        std::fs::remove_file(&db_path)?;
    }
    if is_test && (&realtime_db_path).exists() {
        std::fs::remove_file(&realtime_db_path)?;
    }
    let run_init: bool = is_test || !((&db_path).exists()) || !((&realtime_db_path).exists());
    if run_init {
        // TODO: Log errors
        db_init()?
    }
    Ok(())
}

pub fn db_load(sql_path: &str) -> std::io::Result<()> {
    let bin_target_path: PathBuf = PathBuf::from(format!("{}/target/release/whitebeam", env!("PWD")));
    let _status = Command::new(bin_target_path).args(&["--load", sql_path]).stdout(Stdio::null()).stderr(Stdio::null()).status()?;
    Ok(())
}
