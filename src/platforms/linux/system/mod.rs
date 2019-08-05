// Load hook
#[macro_use]
mod hook;

use ::libc::{c_char, c_int};

hook! {
    unsafe fn hooked_execve(filename: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int => execve {
        // Check /opt/whitebeam/cache.json
        eprintln!("In LD_PRELOAD!");
        real!(hooked_execve)(filename, argv, envp)
    }

}
