// Load OS-specific modules
#[macro_use]
mod hook;
mod system;

use libc::{c_char, c_int};
use std::ffi::CStr;
use crate::library::common::hash;
use crate::library::common::event;
// TODO: Need this?
use std::{os::unix::ffi::OsStringExt};
use std::{ffi::OsString};

fn parse_env(input: &[u8]) -> Option<(OsString, OsString)> {
	if input.is_empty() {
		return None;
	}
	let pos = input[1..].iter().position(|&x| x == b'=').map(|p| p + 1);
	pos.map(|p| {
		(
			OsStringExt::from_vec(input[..p].to_vec()),
			OsStringExt::from_vec(input[p + 1..].to_vec()),
		)
	})
}

fn is_whitelisted(program: &str, env: &Vec<(OsString, OsString)>) -> bool {
    // TODO: Reference /opt/whitebeam/cache.json

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
        // Whitebeam
        ("/opt/whitebeam/whitebeam", false),
        ("/usr/local/bin/whitebeam", false)
    ];
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

hook! {
    unsafe fn hooked_execve(filename: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int => execve {
        // TODO: Garbage collection

        // Program
        let program_c_str: &CStr = CStr::from_ptr(filename);
        let program_str_slice: &str = program_c_str.to_str().unwrap();
        let program: String = program_str_slice.to_owned(); // If necessary

        // Environment variables
        let mut envp_new = envp;
        let mut env: Vec<(OsString, OsString)> = Vec::new();
        while !(*envp_new).is_null() {
                if let Some(key_value) = parse_env(CStr::from_ptr(*envp_new).to_bytes()) {
                    env.push(key_value);
                }
                envp_new = envp_new.offset(1);
        }

        // Program hexdigest
        let hexdigest = hash::common_hash_file(&program);

        // User ID
        let uid = system::get_current_uid();

        // Permit/deny execution
        if is_whitelisted(&program, &env) {
            event::send_exec_event(uid, &program, &hexdigest, true);
            real!(hooked_execve)(filename, argv, envp)
        } else {
            event::send_exec_event(uid, &program, &hexdigest, false);
            *system::errno_location() = system::EACCES;
            return -1
        }
    }
}
