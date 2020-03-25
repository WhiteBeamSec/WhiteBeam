// Load OS-specific modules
#[macro_use]
mod template;
use libc::{c_char, c_int};
use std::env;
use std::ffi::CString;


extern "C" {
    pub fn execl(path: *const c_char, args: ...) -> c_int;
    pub fn execle(path: *const c_char, args: ...) -> c_int;
    pub fn execlp(file: *const c_char, args: ...) -> c_int;
    pub fn execv(path: *const c_char, argv: *const *const c_char) -> c_int;
    pub fn execve(path: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int;
    pub fn execvp(file: *const c_char, argv: *const *const c_char) -> c_int;
    pub fn execvpe(file: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int;
    pub fn fexecve(fd: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int;
}

test_exec_hook! { test execv (test_execv, mod_env: false, mod_path: false) }
test_exec_hook! { test execve (test_execve, mod_env: true, mod_path: false) }
test_exec_hook! { test execvp (test_execvp, mod_env: false, mod_path: true) }
test_exec_hook! { test execvpe (test_execvpe, mod_env: true, mod_path: true) }
test_variadic_exec_hook! { test execl (test_execl, mod_env: false, mod_path: false) }
test_variadic_exec_hook! { test execle (test_execle, mod_env: true, mod_path: false) }
test_variadic_exec_hook! { test execlp (test_execlp, mod_env: false, mod_path: true) }
//test_exec_hook! { test fexecve (test_fexecve, mod_env: false, mod_path: false) }

pub fn run_test(test: &str, test_type: &str) {
    if test == "execv" {
        test_execv(test_type);
    } else if test == "execve" {
        test_execve(test_type);
    } else if test == "execvp" {
        test_execvp(test_type);
    } else if test == "execvpe" {
        test_execvpe(test_type);
    } else if test == "execl" {
        test_execl(test_type);
    } else if test == "execle" {
        test_execle(test_type);
    } else if test == "execlp" {
        test_execlp(test_type);
    } /* else if test == "fexecve" {
        test_fexecve(test_type);
    } */ else {
        eprintln!("WhiteBeam: No test found for {}", test);
        return;
    }
}
