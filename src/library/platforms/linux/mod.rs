// Load OS-specific modules
#[macro_use]
mod hook;
mod hooks;

use libc::{c_char, c_int};
use std::ffi::CStr;
use std::{os::unix::ffi::OsStringExt};
use std::{ffi::OsString};
use crate::library::common::hash;

pub unsafe fn errno_location() -> *mut c_int {
    libc::__errno_location()
}

pub fn get_current_uid() -> u32 {
    unsafe { libc::getuid() }
}

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

unsafe fn transform_parameters(path: *const c_char, envp: *const *const c_char, fd: c_int) -> (String, Vec<(OsString, OsString)>) {
	// TODO: GC

	// Program
	let program = if !(path.is_null()) {
		let program_c_str: &CStr = CStr::from_ptr(path);
		let program_str_slice: &str = program_c_str.to_str().unwrap();
		program_str_slice.to_owned()
	} else {
		format!("fd://{}", fd)
	};

	// Environment variables
	let mut env: Vec<(OsString, OsString)> = Vec::new();
	if !(envp.is_null()) {
		let mut envp_iter = envp;
		while !(*envp_iter).is_null() {
				if let Some(key_value) = parse_env(CStr::from_ptr(*envp_iter).to_bytes()) {
					env.push(key_value);
				}
				envp_iter = envp_iter.offset(1);
		}
	}

	(program, env)
}

fn get_hash_and_uid(program: &str) -> (String, u32) {
	// Hexdigest
	let hexdigest = hash::common_hash_file(&program);

	// User ID
	let uid = get_current_uid();

	(hexdigest, uid)
}

fn is_whitelisted(program: &str, env: &Vec<(OsString, OsString)>) -> bool {
    // TODO: Reference /opt/whitebeam/cache.json, use SHA3-256 hash

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
