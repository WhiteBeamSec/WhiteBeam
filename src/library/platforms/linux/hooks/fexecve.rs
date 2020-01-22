use libc::{c_char, c_int};
use crate::platforms::linux;
use crate::common::whitelist;
use crate::common::event;
use crate::common::hash;
use std::ffi::OsStr;

/*
       int fexecve(int fd, char *const argv[], char *const envp[]);
*/
hook! {
    unsafe fn hooked_fexecve(fd: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int => fexecve {
		let program = OsStr::new("fd");
        let env = linux::parse_env_collection(envp);
        let hexdigest = hash::common_hash_fd(fd);
        let uid = linux::get_current_uid();
        // Permit/deny execution
        if whitelist::is_whitelisted(program, &env, &hexdigest) {
            event::send_exec_event(uid, program, &hexdigest, true);
            real!(hooked_fexecve)(fd, argv, envp)
        } else {
            event::send_exec_event(uid, program, &hexdigest, false);
            *linux::errno_location() = libc::EACCES;
            return -1
        }
    }
}
