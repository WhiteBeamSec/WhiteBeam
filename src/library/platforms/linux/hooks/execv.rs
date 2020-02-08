use libc::{c_char, c_int};
use std::ptr;
use crate::platforms::linux;
use crate::common::whitelist;
use crate::common::event;
use crate::common::hash;

/*
       int execv(const char *path, char *const argv[]);
*/
#[no_mangle]
pub unsafe extern "C" fn execv(path: *const c_char, argv: *const *const c_char) -> c_int {
	let envp: *const *const c_char = ptr::null();
    let program = linux::c_char_to_osstring(path);
    let env = linux::parse_env_collection(envp);
    let hexdigest = hash::common_hash_file(&program);
    let uid = linux::get_current_uid();
    // Permit/deny execution
    if whitelist::is_whitelisted(&program, &env, &hexdigest) {
        event::send_exec_event(uid, &program, &hexdigest, true);
        // Pass through
        static mut REAL: *const u8 = 0 as *const u8;
        static mut ONCE: ::std::sync::Once = ::std::sync::Once::new();
        ONCE.call_once(|| {
            REAL = crate::platforms::linux::dlsym_next("execv\u{0}");
        });
        let execv_real: unsafe extern "C" fn(path: *const c_char, argv: *const *const c_char) -> c_int = std::mem::transmute(REAL);
        execv_real(path, argv)
    } else {
        event::send_exec_event(uid, &program, &hexdigest, false);
        *linux::errno_location() = libc::EACCES;
        return -1
    }
}
