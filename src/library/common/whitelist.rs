//use std::env;
#[cfg(target_os = "windows")]
use crate::library::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::library::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::library::platforms::macos as platform;
use std::{ffi::OsString};

pub fn is_whitelisted(program: &str, env: &Vec<(OsString, OsString)>) -> bool {
    // TODO: Use SHA3-256 hash

    let mut unsafe_env = false;
    let mut allow_exec = false;
    let env_blacklist = [
            "LD_PRELOAD",
            "LD_AUDIT",
            "LD_LIBRARY_PATH"
    ];
    let whitelist = [
        // Tuple of (permitted program, allow unsafe environment variables)
        // Shells
        ("/bin/bash", false),
        ("/bin/sh", false),
        // WhiteBeam
        ("/opt/WhiteBeam/whitebeam", false),
        ("/usr/local/bin/whitebeam", false)
    ];
    // TODO: Parse cache file
    let _cache_file = platform::get_cache_file();
    for env_var in env {
        if env_blacklist.contains(&env_var.0.to_str().unwrap()) {
            unsafe_env = true;
            break;
        }
    }
    for (allowed_program, allow_unsafe) in whitelist.iter() {
        if (&program == allowed_program) && (&unsafe_env == allow_unsafe) {
            allow_exec = true;
            break;
        }
    }
    allow_exec
}
