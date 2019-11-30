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
        // Ensure legacy versions of man-db use WhiteBeam versus seccomp
        /*
        if program == "/usr/bin/man" {
            // Add MAN_DISABLE_SECCOMP=1 to envp
            let mut env_vec: Vec<*const c_char> = Vec::new();
            let mut next_argv: isize = args.arg();
            let mut ptr_to_next_argv = next_argv as *const c_char;
            while !(ptr_to_next_argv).is_null() {
                arg_vec.push(ptr_to_next_argv);
                next_argv = args.arg();
                ptr_to_next_argv = next_argv as *const c_char;
            }
            arg_vec.push(std::ptr::null());
            let argv: *const *const c_char = (&arg_vec).as_ptr() as *const *const c_char;
        }
        */
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
