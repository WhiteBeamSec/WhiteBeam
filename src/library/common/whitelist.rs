#[cfg(target_os = "windows")]
use crate::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::platforms::macos as platform;
use crate::common::db;
use std::{ffi::OsStr, ffi::OsString};

// Hardcoded whitelist data for setup
fn get_hardcoded_env_blacklist() -> Vec<OsString> {
    vec!(
        OsString::from("LD_PRELOAD"),
        OsString::from("LD_AUDIT"),
        OsString::from("LD_LIBRARY_PATH")
    )
}

fn get_hardcoded_whitelist() -> Vec<(OsString, bool, String)> {
    #[cfg(feature = "whitelist_test")]
    return vec!(
        (OsString::from("/usr/bin/whoami"), true, String::from("ANY")),
        // Test seccomp
        (OsString::from("/usr/bin/man"), true, String::from("ANY"))
    );
    #[cfg(not(feature = "whitelist_test"))]
    return vec!(
        // Tuple of (permitted program, allow unsafe environment variables, SHA-512 hexdigest)
        // Shells
        (OsString::from("/bin/bash"), false, String::from("ANY")),
        (OsString::from("/bin/sh"), false, String::from("ANY")),
        // WhiteBeam
        (OsString::from("/opt/WhiteBeam/whitebeam"), false, String::from("ANY")),
        (OsString::from("/usr/local/bin/whitebeam"), false, String::from("ANY"))
    )
}

pub fn is_whitelisted(program: &OsStr, env: &Vec<(OsString, OsString)>, hexdigest: &str) -> bool {
    let hardcoded_env_blacklist = get_hardcoded_env_blacklist();
    let hardcoded_whitelist = get_hardcoded_whitelist();
    let mut unsafe_env = false;
    for env_var in env {
        if hardcoded_env_blacklist.contains(&env_var.0) {
            unsafe_env = true;
            break;
        }
    }
    // Permit hardcoded application whitelist
    for (allowed_program, allow_unsafe, allowed_hash) in hardcoded_whitelist.iter() {
        if  (&program == allowed_program) &&
            (&unsafe_env == allow_unsafe) &&
            ((&hexdigest == allowed_hash) || (allowed_hash == "ANY")) {
            return true;
        }
    }
    if cfg!(feature = "whitelist_test") {
        return false;
    }
    // Introduced limitation:
    // WhiteBeam is permissive for up to 5 minutes after boot to avoid interfering with the boot
    // process. While attackers should not be able to reboot a system due to whitelisting policy,
    // this is a weakness while WhiteBeam is actively developed. Alternatives include:
    // 1. Whitelisting all binaries by default, including malware (other EDR software use
    //    this approach, maintaining a large database of permitted executables)
    // 2. Require a reboot to baseline systems (which may interfere with production systems)
    // Feedback/ideas welcome: https://github.com/WhiteBeamSec/WhiteBeam/issues
    match platform::get_uptime() {
        Ok(uptime) => {
            if uptime.as_secs() < (60*5) {
                return true;
            }
        },
        Err(e) => eprintln!("WhiteBeam: {}", e)
    };
    let conn = match db::db_open() {
        Ok(c) => c,
        Err(e) => {
            // No dynamic whitelist present, deny by default
            eprintln!("WhiteBeam: {}", e);
            return false;
        }
    };
    // Permit execution if running in disabled mode
    if !(db::get_enabled(&conn)) {
        return true;
    }
    // Permit authorized execution
    if db::get_valid_auth_env(&conn) {
        return true;
    }
    // Permit user application whitelist
    for dyn_result in db::get_dyn_whitelist(&conn).unwrap_or(Vec::new()).iter() {
        if  (&program == &OsStr::new(&dyn_result.program)) &&
            (&unsafe_env == &dyn_result.allow_unsafe) &&
            ((&hexdigest == &dyn_result.hash) || (&dyn_result.hash == &"ANY")) {
            return true;
        }
    }
    // Deny by default
    false
}
