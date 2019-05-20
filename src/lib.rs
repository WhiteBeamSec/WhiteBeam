#[macro_use]
pub mod platforms;

use ::libc::{c_char, c_void, uid_t};

hook! {
    pub extern "C" unsafe fn hooked_getuid() -> uid_t => getuid {
        println!("In LD_PRELOAD!");
        return 0;
    }
}
