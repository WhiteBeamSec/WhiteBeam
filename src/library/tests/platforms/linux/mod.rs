// Load OS-specific modules
#[macro_use]
mod template;
use libc::{execv, execve, execvp, execvpe, execl, execle, execlp, fexecve};
use std::{env, ffi::CString};

test_exec_hook! { test execv (test_execv, mod_env: false, mod_path: false) }
test_exec_hook! { test execve (test_execve, mod_env: true, mod_path: false) }
test_exec_hook! { test execvp (test_execvp, mod_env: false, mod_path: true) }
test_exec_hook! { test execvpe (test_execvpe, mod_env: true, mod_path: true) }
test_variadic_exec_hook! { test execl (test_execl, mod_env: false, mod_path: false) }
test_variadic_exec_hook! { test execle (test_execle, mod_env: true, mod_path: false) }
test_variadic_exec_hook! { test execlp (test_execlp, mod_env: false, mod_path: true) }
//test_exec_hook! { test fexecve (test_fexecve, mod_env: false, mod_path: false) }

pub fn run_tests() {
    // TODO: Refactor to be similar to action framework
    let tests: Vec<&str> = vec!["positive", "negative"];
    let modules: Vec<&str> = vec!["execl", "execle", "execlp", "execv", "execve", "execvp", "execvpe", "fexecve"];
    #[cfg(not(target_os = "linux"))]
    unimplemented!("WhiteBeam: Tests on non-Linux platforms are not currently supported");
    for module in modules.clone() {
        // TODO: fexecve in Linux tests
        if module == "fexecve" {
            eprintln!("WhiteBeam: Skipping fexecve");
            continue;
        }
        for test_type in tests.clone() {
            let exit_status_child = match module {
                "execv" => test_execv(test_type),
                "execve" => test_execve(test_type),
                "execvp" => test_execvp(test_type),
                "execvpe" => test_execvpe(test_type),
                "execl" => test_execl(test_type),
                "execle" => test_execle(test_type),
                "execlp" => test_execlp(test_type),
                //"fexecve" => test_fexecve(test_type),
                _ => {eprintln!("WhiteBeam: No test found for {}", &module); -1}
            };
            if test_type == "positive" {
                // Positive test
                assert!(exit_status_child >= 0);
                let contents = std::fs::read_to_string("/tmp/test_result").expect("WhiteBeam: Could not read test result file");
                assert_eq!(contents, format!("{}/target/release/libwhitebeam.so", env!("PWD")));
                std::fs::remove_file("/tmp/test_result").expect("WhiteBeam: Failed to remove /tmp/test_result");
            } else {
                // Negative test
                // TODO: assert!(!exit_status_module.success());
                assert_eq!(std::path::Path::new("/tmp/test_result").exists(), false);
            }
            println!("WhiteBeam: {} passed ({} test).", module, test_type);
        }
    }
    println!("WhiteBeam: All tests passed")
}
