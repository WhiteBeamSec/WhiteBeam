use libc::{c_char, c_int};
use std::ptr;
use crate::platforms::linux;
use crate::common::whitelist;
use crate::common::event;
use crate::common::hash;

/*
       int execv(const char *path, char *const argv[]);
*/
hook! {
    unsafe fn hooked_execv(path: *const c_char, argv: *const *const c_char) -> c_int => execv {
		let envp: *const *const c_char = ptr::null();
        let program = linux::c_char_to_osstring(path);
        let env = linux::parse_env_collection(envp);
        let hexdigest = hash::common_hash_file(&program);
        let uid = linux::get_current_uid();
        // Permit/deny execution
        if whitelist::is_whitelisted(&program, &env, &hexdigest) {
            event::send_exec_event(uid, &program, &hexdigest, true);
            real!(hooked_execv)(path, argv)
        } else {
            event::send_exec_event(uid, &program, &hexdigest, false);
            *linux::errno_location() = libc::EACCES;
            return -1
        }
    }
}
