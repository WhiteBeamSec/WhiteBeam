use libc::{c_char, c_int};
use std::ffi::OsString;
use crate::platforms::linux;
use crate::common::whitelist;
use crate::common::event;
use crate::common::hash;

/*
       int execve(const char *path, char *const argv[],
                  char *const envp[]);
*/
hook! {
    unsafe fn hooked_execve(path: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int => execve {
        let program = linux::c_char_to_osstring(path);
        let env = linux::parse_env_collection(envp);
        let hexdigest = hash::common_hash_file(&program);
        let uid = linux::get_current_uid();
        // Warn that legacy versions of man-db must disable seccomp
        if program == "/usr/bin/man" {
            let needle = OsString::from("MAN_DISABLE_SECCOMP");
            let mut disable_defined = false;
            for env_var in &env {
                if env_var.0 == needle {
                    disable_defined = true;
                    break;
                }
            }
            if !(disable_defined) {
                eprintln!("WhiteBeam: Legacy man-db versions require MAN_DISABLE_SECCOMP=1")
            }
        }
        // Permit/deny execution
        if whitelist::is_whitelisted(&program, &env, &hexdigest) {
            event::send_exec_event(uid, &program, &hexdigest, true);
            real!(hooked_execve)(path, argv, envp)
        } else {
            event::send_exec_event(uid, &program, &hexdigest, false);
            *linux::errno_location() = libc::EACCES;
            return -1
        }
    }
}
