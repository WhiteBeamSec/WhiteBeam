use libc::{c_char, c_int};
use std::ptr;
use crate::library::platforms::linux;
use crate::library::common::whitelist;
use crate::library::common::event;

/*
       int execl(const char *path, const char *arg, ...
                       /* (char  *) NULL */);
*/
/*
TODO: On Linux, argv and envp can be specified as NULL.  In both cases,
      this has the same effect as specifying the argument as a pointer to a
      list containing a single null pointer.  Do not take advantage of this
      nonstandard and nonportable misfeature!  On many other UNIX systems,
      specifying argv as NULL will result in an error (EFAULT).  Some other
      UNIX systems treat the envp==NULL case the same as Linux.
*/
#[no_mangle]
pub unsafe extern "C" fn execl(path: *const c_char, mut args: ...) -> c_int {
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
    let envp: *const *const c_char = ptr::null();

    let (program, env) = linux::transform_parameters(path, envp, -1);
    let (hexdigest, uid) = linux::get_hash_and_uid(&program);

    // Permit/deny execution
    if whitelist::is_whitelisted(&program, &env, &hexdigest) {
        event::send_exec_event(uid, &program, &hexdigest, true);
        // Call execve
        static mut REAL: *const u8 = 0 as *const u8;
        static mut ONCE: ::std::sync::Once = ::std::sync::Once::new();
        ONCE.call_once(|| {
            REAL = crate::library::platforms::linux::hook::dlsym_next("execve\u{0}");
        });
        let execve: unsafe extern "C" fn(path: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int = std::mem::transmute(REAL);
        execve(path, argv, envp)
    } else {
        event::send_exec_event(uid, &program, &hexdigest, false);
        *linux::errno_location() = libc::EACCES;
        return -1
    }
}
