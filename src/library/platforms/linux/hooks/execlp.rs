use libc::{c_char, c_int};
use std::env;
use std::path::PathBuf;
use std::ptr;
use std::{ffi::CString,
          os::unix::ffi::OsStrExt};
use crate::platforms::linux;
use crate::common::whitelist;
use crate::common::event;
use crate::common::hash;

/*
       int execlp(const char *path, const char *arg, ...
                       /* (char  *) NULL */);
*/
#[no_mangle]
pub unsafe extern "C" fn execlp(path: *const c_char, mut args: ...) -> c_int {
    // Populate argv
    let mut arg_vec: Vec<*const c_char> = Vec::new();
    let mut next_argv: isize = args.arg();
    let mut ptr_to_next_argv = next_argv as *const c_char;
	while !(ptr_to_next_argv).is_null() {
		arg_vec.push(ptr_to_next_argv);
        next_argv = args.arg();
        ptr_to_next_argv = next_argv as *const c_char;
	}
    arg_vec.push(std::ptr::null());
    let argv: *const *const c_char = (&arg_vec).as_ptr() as *const *const c_char;

    // Populate envp
    // TODO: Don't use null (only supported by Linux)
    let envp: *const *const c_char = ptr::null();

    let program = linux::c_char_to_osstring(path);
    let env = linux::parse_env_collection(envp);
    let which_abs_pathbuf = match which::which_in(&program,
                                                  Some(linux::get_env_path()),
                                                  env::current_dir().unwrap_or(PathBuf::new())) {
        Err(_why) => {
            *linux::errno_location() = libc::ENOENT;
            return -1 },
        Ok(prog_path) => prog_path
    };
    let abs_prog_str = which_abs_pathbuf.as_os_str();
    let hexdigest = hash::common_hash_file(abs_prog_str);
    let uid = linux::get_current_uid();

    // Populate path
    let abs_path_c_str = match CString::new(abs_prog_str.as_bytes()) {
        Err(_why) => {
            *linux::errno_location() = libc::ENOENT;
            return -1 },
        Ok(abs_prog_path) => abs_prog_path
    };
    let abs_path_c_char: *const c_char = abs_path_c_str.as_ptr() as *const c_char;

    // Permit/deny execution
    if whitelist::is_whitelisted(abs_prog_str, &env, &hexdigest) {
        event::send_exec_event(uid, abs_prog_str, &hexdigest, true);
        // Call execve
        static mut REAL: *const u8 = 0 as *const u8;
        static mut ONCE: ::std::sync::Once = ::std::sync::Once::new();
        ONCE.call_once(|| {
            REAL = crate::platforms::linux::hook::dlsym_next("execve\u{0}");
        });
        let execve: unsafe extern "C" fn(path: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int = std::mem::transmute(REAL);
        execve(abs_path_c_char, argv, envp)
    } else {
        event::send_exec_event(uid, abs_prog_str, &hexdigest, false);
        *linux::errno_location() = libc::EACCES;
        return -1
    }
}
