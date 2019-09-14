use libc::{c_char, c_int};
use std::ptr;
use crate::library::platforms::linux;
use crate::library::common::event;

/*
       int fexecve(int fd, char *const argv[], char *const envp[]);
*/
hook! {
    unsafe fn hooked_fexecve(fd: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int => fexecve {
		let path: *const c_char = ptr::null();
		let (program, env) = linux::transform_parameters(path, envp, fd);
		let (hexdigest, uid) = linux::get_hash_and_uid(&program);
        // Permit/deny execution
        if linux::is_whitelisted(&program, &env) {
            event::send_exec_event(uid, &program, &hexdigest, true);
            real!(hooked_fexecve)(fd, argv, envp)
        } else {
            event::send_exec_event(uid, &program, &hexdigest, false);
            *linux::errno_location() = libc::EACCES;
            return -1
        }
    }
}
