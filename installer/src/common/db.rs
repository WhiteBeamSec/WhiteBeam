#[cfg(target_os = "windows")]
use crate::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::platforms::macos as platform;
use std::{error::Error,
          io::Write,
          path::PathBuf,
          process::Command,
          process::Stdio};

fn db_init() -> Result<(), Box<dyn Error>> {
    db_load("Schema")?;
    db_load("Default")?;
    let bin_target_path: PathBuf = PathBuf::from(format!("{}/target/release/whitebeam", env!("PWD")));
    let mut child = Command::new(bin_target_path).args(&["--setting", "SystemArchitecture", std::env::consts::ARCH]).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    // TODO: _output, debugging information follows:
    let output = child.wait_with_output()?;
    print!("stdout: {}", std::str::from_utf8(&output.stdout).unwrap());
    if output.stderr.len() > 0 {
        eprint!("stderr: {}", std::str::from_utf8(&output.stderr).unwrap());
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
    let mut child = Command::new(bin_target_path).args(&["--load", sql_path]).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    // TODO: _output, debugging information follows:
    let output = child.wait_with_output()?;
    print!("stdout: {}", std::str::from_utf8(&output.stdout).unwrap());
    if output.stderr.len() > 0 {
        eprint!("stderr: {}", std::str::from_utf8(&output.stderr).unwrap());
    }
    Ok(())
}
