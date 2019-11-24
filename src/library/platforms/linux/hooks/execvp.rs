use libc::{c_char, c_int};
use std::ptr;
use crate::library::platforms::linux;
use crate::library::common::whitelist;
use crate::library::common::event;

/*
       int execvp(const char *file, char *const argv[]);
*/
hook! {
    unsafe fn hooked_execvp(path: *const c_char, argv: *const *const c_char) -> c_int => execvp {
		let envp: *const *const c_char = ptr::null();
		let (program, env) = linux::transform_parameters(path, envp, -1);
		let which_abs_pathbuf = match which::which(&program) {
            Err(_why) => {
				*linux::errno_location() = libc::ENOENT;
				return -1 },
            Ok(prog_path) => prog_path
        };
		let abs_prog_str = which_abs_pathbuf.to_str().unwrap();
		let (hexdigest, uid) = linux::get_hash_and_uid(abs_prog_str);
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
