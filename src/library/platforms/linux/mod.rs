// Load OS-specific modules
#[macro_use]
mod hook;
mod hooks;

use libc::{c_char, c_int};
use std::{env,
          mem,
          ffi::CStr,
          ffi::OsString,
          os::unix::ffi::OsStringExt,
          path::Path,
          path::PathBuf,
          time::Duration};
use crate::library::common::hash;

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    let data_path: String = "/opt/WhiteBeam/data/".to_owned();
    let data_file_path = data_path + data_file;
    Path::new(&data_file_path).to_owned()
}

pub fn get_uptime() -> Result<Duration, String> {
    let mut info: libc::sysinfo = unsafe { mem::zeroed() };
    let ret = unsafe { libc::sysinfo(&mut info) };
    if ret == 0 {
        Ok(Duration::from_secs(info.uptime as u64))
    } else {
        Err("sysinfo() failed".to_string())
    }
}

pub unsafe fn errno_location() -> *mut c_int {
    libc::__errno_location()
}

pub fn get_current_uid() -> u32 {
    unsafe { libc::getuid() }
}

pub fn get_env_path() -> OsString {
    let path_val: OsString;
    match env::var_os("PATH") {
        Some(val) => {
            path_val = val;
        }
        None => {
            // Use CS_PATH
            path_val = OsString::from("/bin:/usr/bin");
        }
    }
    path_val
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
