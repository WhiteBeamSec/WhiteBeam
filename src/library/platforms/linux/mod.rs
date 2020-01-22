// Load OS-specific modules
#[macro_use]
mod hook;
mod hooks;

use libc::{c_char, c_int};
use std::{env,
          mem,
          ffi::CStr,
          ffi::OsStr,
          ffi::OsString,
          os::unix::ffi::OsStrExt,
          os::unix::ffi::OsStringExt,
          path::Path,
          path::PathBuf,
          time::Duration};

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    let data_path: String = String::from("/opt/WhiteBeam/data/");
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
    match env::var_os("PATH") {
        Some(val) => {
            val
        }
        None => {
            // Use CS_PATH
            OsString::from("/bin:/usr/bin")
        }
    }
}

fn parse_env_single(input: &[u8]) -> Option<(OsString, OsString)> {
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

unsafe fn parse_env_collection(envp: *const *const c_char) -> Vec<(OsString, OsString)> {
    let mut env: Vec<(OsString, OsString)> = Vec::new();
    if !(envp.is_null()) {
        let mut envp_iter = envp;
        while !(*envp_iter).is_null() {
                if let Some(key_value) = parse_env_single(CStr::from_ptr(*envp_iter).to_bytes()) {
                    env.push(key_value);
                }
                envp_iter = envp_iter.offset(1);
        }
    }
    env
}

unsafe fn c_char_to_osstring(char_ptr: *const c_char) -> OsString {
    match char_ptr.is_null() {
        true => OsString::new(),
        false => {
            let program_c_str: &CStr = CStr::from_ptr(char_ptr);
    		OsStr::from_bytes(program_c_str.to_bytes()).to_owned()
        }
    }
}
