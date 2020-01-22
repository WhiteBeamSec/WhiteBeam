use libc::{c_char, c_int};
use std::env;
use std::ptr;
use crate::platforms::linux;
use crate::common::whitelist;
use crate::common::event;
use crate::common::hash;

/*
       int execvp(const char *file, char *const argv[]);
*/
hook! {
    unsafe fn hooked_execvp(path: *const c_char, argv: *const *const c_char) -> c_int => execvp {
		let envp: *const *const c_char = ptr::null();
        let program = linux::c_char_to_osstring(path);
        let env = linux::parse_env_collection(envp);
		let which_abs_pathbuf = match which::which_in(&program,
                                                      Some(linux::get_env_path()),
                                                      env::current_dir().unwrap()) {
            Err(_why) => {
				*linux::errno_location() = libc::ENOENT;
				return -1 },
            Ok(prog_path) => prog_path
        };
		let abs_prog_str = which_abs_pathbuf.as_os_str();
        let hexdigest = hash::common_hash_file(abs_prog_str);
        let uid = linux::get_current_uid();
        // Permit/deny execution
        if whitelist::is_whitelisted(abs_prog_str, &env, &hexdigest) {
            event::send_exec_event(uid, abs_prog_str, &hexdigest, true);
            real!(hooked_execvp)(path, argv)
        } else {
            event::send_exec_event(uid, abs_prog_str, &hexdigest, false);
            *linux::errno_location() = libc::EACCES;
            return -1
        }
    }
}
