// Load OS-specific modules
// TODO: AppInit_DLLs or Detours https://github.com/Microsoft/Detours/wiki

//use std::env;
use std::path::Path;

pub fn get_cache_file() -> &'static Path {
    // TODO: Change this when registry and environment are secured
    //Path::new(env::var("ProgramFiles").unwrap().push_str("\\WhiteBeam\\cache.json"))
    Path::new("C:\\Program Files\\WhiteBeam\\data\\cache.json")
}
