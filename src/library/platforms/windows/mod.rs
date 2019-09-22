// Load OS-specific modules
// TODO: AppInit_DLLs or Detours https://github.com/Microsoft/Detours/wiki

//use std::env;
use std::{path::Path,
          time::Duration};

pub fn get_cache_file() -> &'static Path {
    // TODO: Change this when registry and environment are secured
    //Path::new(env::var("ProgramFiles").unwrap().push_str("\\WhiteBeam\\cache.json"))
    Path::new("C:\\Program Files\\WhiteBeam\\data\\cache.json")
}

pub fn get_uptime() -> Result<Duration, String> {
    let ret: u64 = unsafe { kernel32::GetTickCount64() };
    Ok(Duration::from_millis(ret))
}
