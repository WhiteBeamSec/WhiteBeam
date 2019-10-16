use libc::{c_char, c_int};
use crate::library::platforms::linux;
use crate::library::common::whitelist;
use crate::library::common::event;

#[no_mangle]
pub unsafe extern "C" fn execle(path: *const c_char, mut args: ...) -> c_int {
    // Populate argv
    let mut arg_vec: Vec<*const c_char> = Vec::new();
    let mut next_argv: isize = args.arg();
    let mut ptr_to_next_argv = next_argv as *const c_char;
	while !(ptr_to_next_argv).is_null() {
		arg_vec.push(ptr_to_next_argv);
        next_argv = args.arg();
        ptr_to_next_argv = next_argv as *const c_char;
	}
    arg_vec.push(std::ptr::null());
    let argv: *const *const c_char = (&arg_vec).as_ptr() as *const *const c_char;

    // Populate envp
    let envp_arg: isize = args.arg();
    let envp = envp_arg as *const *const c_char;

    // Call execve
    static mut REAL: *const u8 = 0 as *const u8;
    static mut ONCE: ::std::sync::Once = ::std::sync::Once::new();
    unsafe {
        ONCE.call_once(|| {
            REAL = crate::library::platforms::linux::hook::dlsym_next("execve\u{0}");
        });
    }
    let execve: unsafe extern "C" fn(path: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int = unsafe { std::mem::transmute(REAL) };
    execve(path, argv, envp)
}
