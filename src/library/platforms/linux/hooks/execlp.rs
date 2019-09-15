use libc::{c_char, c_int};
use std::ptr;
use crate::library::platforms::linux;
use crate::library::common::whitelist;
use crate::library::common::event;

/*
       int execlp(const char *path, const char *arg, ...
                       /* (char  *) NULL */);
*/
hook! {
    unsafe fn hooked_execlp(path: *const c_char, arg: *const c_char) -> c_int => execlp {
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
        if whitelist::is_whitelisted(&abs_prog_str, &env) {
            event::send_exec_event(uid, &abs_prog_str, &hexdigest, true);
            real!(hooked_execlp)(path, arg)
        } else {
            event::send_exec_event(uid, &abs_prog_str, &hexdigest, false);
            *linux::errno_location() = libc::EACCES;
            return -1
        }
    }
}
