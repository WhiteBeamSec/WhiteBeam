// Load OS-specific modules
// TODO: AppCert DLLs

//use std::env;
use std::{path::Path,
          path::PathBuf,
          time::Duration};

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    // TODO: Change this when registry and environment are secured
    //Path::new(env::var("ProgramFiles").unwrap_or("C:\\ProgramFiles").push_str("\\WhiteBeam\\data\\"))
    let data_path: String = String::from("C:\\Program Files\\WhiteBeam\\data\\");
    let data_file_path = data_path + data_file;
    Path::new(&data_file_path).to_owned()
}

pub fn get_uptime() -> Result<Duration, String> {
    let ret: u64 = unsafe { kernel32::GetTickCount64() };
    Ok(Duration::from_millis(ret))
}
