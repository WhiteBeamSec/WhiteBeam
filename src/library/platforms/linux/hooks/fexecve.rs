use libc::{c_char, c_int};
use crate::platforms::linux;
use crate::common::whitelist;
use crate::common::event;
use crate::common::hash;
use std::ffi::OsStr;

/*
       int fexecve(int fd, char *const argv[], char *const envp[]);
*/
#[no_mangle]
pub unsafe extern "C" fn fexecve(fd: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int {
	let program = OsStr::new("fd");
    let env = linux::parse_env_collection(envp);
    let hexdigest = hash::common_hash_fd(fd);
    let uid = linux::get_current_uid();
    // Permit/deny execution
    if whitelist::is_whitelisted(program, &env, &hexdigest) {
        event::send_exec_event(uid, program, &hexdigest, true);
        // Pass through
        static mut REAL: *const u8 = 0 as *const u8;
        static mut ONCE: ::std::sync::Once = ::std::sync::Once::new();
        ONCE.call_once(|| {
            REAL = crate::platforms::linux::dlsym_next("fexecve\u{0}");
        });
        let fexecve_real: unsafe extern "C" fn(fd: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int = std::mem::transmute(REAL);
        fexecve_real(fd, argv, envp)
    } else {
        event::send_exec_event(uid, program, &hexdigest, false);
        *linux::errno_location() = libc::EACCES;
        return -1
    }
}
