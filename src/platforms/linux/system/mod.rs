// Load hook
#[macro_use]
mod hook;

use ::libc::{c_char, c_void, c_int};

hook! {
    pub extern "C" unsafe fn hooked_execve() -> c_int => execve {
        println!("In LD_PRELOAD!");
        // TODO: dlsym execve and return the original function after logging argv with println
        return 0;
    }
}
