use libc::{c_char, c_int};
use crate::library::platforms::linux;
use crate::library::common::whitelist;
use crate::library::common::event;

/*
       int execve(const char *path, char *const argv[],
                  char *const envp[]);
*/
hook! {
    unsafe fn hooked_execve(path: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int => execve {
		let (program, env) = linux::transform_parameters(path, envp, -1);
		let (hexdigest, uid) = linux::get_hash_and_uid(&program);
        // Warn that legacy versions of man-db must disable seccomp
        if program == "/usr/bin/man" {
            let mut disable_defined = false;
            for env_var in &env {
                if &env_var.0.to_str().unwrap() == &"MAN_DISABLE_SECCOMP" {
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
