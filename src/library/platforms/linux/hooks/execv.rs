use libc::{c_char, c_int};
use std::ptr;
use crate::library::platforms::linux;
use crate::library::common::whitelist;
use crate::library::common::event;

/*
       int execv(const char *path, char *const argv[]);
*/
hook! {
    unsafe fn hooked_execv(path: *const c_char, argv: *const *const c_char) -> c_int => execv {
		let envp: *const *const c_char = ptr::null();
		let (program, env) = linux::transform_parameters(path, envp, -1);
		let (hexdigest, uid) = linux::get_hash_and_uid(&program);
        // Permit/deny execution
        if whitelist::is_whitelisted(&program, &env) {
            event::send_exec_event(uid, &program, &hexdigest, true);
            real!(hooked_execv)(path, argv)
        } else {
            event::send_exec_event(uid, &program, &hexdigest, false);
            *linux::errno_location() = libc::EACCES;
            return -1
        }
    }
}
