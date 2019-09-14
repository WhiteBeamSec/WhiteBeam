use libc::{c_char, c_int};
use std::ptr;
use crate::library::platforms::linux;
use crate::library::common::event;

/*
       int execl(const char *path, const char *arg, ...
                       /* (char  *) NULL */);
*/
hook! {
    unsafe fn hooked_execl(path: *const c_char, arg: *const c_char) -> c_int => execl {
		let envp: *const *const c_char = ptr::null();
		let (program, env) = linux::transform_parameters(path, envp, -1);
		let (hexdigest, uid) = linux::get_hash_and_uid(&program);
        // Permit/deny execution
        if linux::is_whitelisted(&program, &env) {
            event::send_exec_event(uid, &program, &hexdigest, true);
            real!(hooked_execl)(path, arg)
        } else {
            event::send_exec_event(uid, &program, &hexdigest, false);
            *linux::errno_location() = libc::EACCES;
            return -1
        }
    }
}
