// Load OS-specific modules
// TODO: AppCert DLLs

//use std::env;
use std::path::PathBuf;

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    // TODO: Use PWD for Powershell with feature="whitelist_test"?
    // TODO: May change this when registry and environment are secured
    //PathBuf::from(env::var("ProgramFiles").unwrap_or("C:\\ProgramFiles").push_str("\\WhiteBeam\\data\\"))
    #[cfg(feature = "whitelist_test")]
    let data_path: String = format!("{}\\target\\release\\examples\\data\\", env!("CD"));
    #[cfg(not(feature = "whitelist_test"))]
    let data_path: String = String::from("C:\\Program Files\\WhiteBeam\\data\\");
    let data_file_path = data_path + data_file;
    PathBuf::from(data_file_path)
}

pub fn get_realtime_file_path(realtime_file: &str) -> PathBuf {
    // TODO: Use PWD for Powershell with feature="whitelist_test"?
    // TODO: May change this when registry and environment are secured
    //PathBuf::from(env::var("ProgramFiles").unwrap_or("C:\\ProgramFiles").push_str("\\WhiteBeam\\realtime\\"))
    #[cfg(feature = "whitelist_test")]
    let realtime_path: String = format!("{}\\target\\release\\examples\\realtime\\", env!("CD"));
    #[cfg(not(feature = "whitelist_test"))]
    let realtime_path: String = String::from("C:\\Program Files\\WhiteBeam\\realtime\\");
    let realtime_file_path = realtime_path + realtime_file;
    PathBuf::from(realtime_file_path)
}

pub fn gettid() -> u64 {
    // Requires winapi crate
    unimplemented!("WhiteBeam: Retrieving thread ID is not currently supported on Windows");
    //unsafe { winapi::um::processthreadsapi::GetCurrentThreadId().into() }
}