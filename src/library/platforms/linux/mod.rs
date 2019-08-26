// Load OS-specific modules
#[macro_use]
mod hook;
mod system;

use libc::{c_char, c_int};
use std::ffi::CStr;
use crate::library::common::hash;
use crate::library::common::event;

hook! {
    unsafe fn hooked_execve(filename: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int => execve {
        // TODO: Check /opt/whitebeam/cache.json for whitelist status
        let allow_exec = true;
        // TODO: Garbage collection here
        let c_str: &CStr = CStr::from_ptr(filename);
        let str_slice: &str = c_str.to_str().unwrap();
        let program: String = str_slice.to_owned(); // If necessary
        let hexdigest = hash::common_hash_file(&program);
        let uid = system::get_current_uid();
        if allow_exec {
            event::send_exec_event(uid, &program, &hexdigest, allow_exec);
            real!(hooked_execve)(filename, argv, envp)
        } else {
            event::send_exec_event(uid, &program, &hexdigest, allow_exec);
            *system::errno_location() = system::EACCES;
            return -1
        }
    }
}
