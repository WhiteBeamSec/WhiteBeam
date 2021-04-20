// Load OS-specific modules
#[macro_use]
mod template;
use libc::{execv, execve, execvp, execvpe, execl, execle, execlp, fexecve,
           creat, creat64, fdopen, fopen, fopen64, open, open64, openat, openat64,
           chmod, fchmod, fchmodat, chown, lchown, fchown, fchownat,
           link, linkat, symlink, symlinkat, rename, renameat, renameat2,
           rmdir, unlink, unlinkat, truncate, ftruncate, mknod, mknodat};
use std::os::linux::fs::MetadataExt;
use std::{env, ffi::CString};

test_exec_hook! { test execv (test_execv, mod_env: false, mod_path: false) }
test_exec_hook! { test execve (test_execve, mod_env: true, mod_path: false) }
test_exec_hook! { test execvp (test_execvp, mod_env: false, mod_path: true) }
test_exec_hook! { test execvpe (test_execvpe, mod_env: true, mod_path: true) }
test_variadic_exec_hook! { test execl (test_execl, mod_env: false, mod_path: false) }
test_variadic_exec_hook! { test execle (test_execle, mod_env: true, mod_path: false) }
test_variadic_exec_hook! { test execlp (test_execlp, mod_env: false, mod_path: true) }
test_exec_hook! { test fexecve (test_fexecve, mod_env: true, mod_path: false) }
// TODO: For Filesystem tests check test_type, /var/tmp or /dev/shm for denied writes
fn test_creat(test_type: &str) -> i32 {
    let _e = std::fs::remove_file("/tmp/test_result_fs");
    unsafe { libc::creat("/tmp/test_result_fs\0".as_ptr() as *const libc::c_char, libc::S_IRUSR | libc::S_IWUSR) }
    // TODO: Close fd
}
fn test_creat64(test_type: &str) -> i32 {
    let _e = std::fs::remove_file("/tmp/test_result_fs");
    unsafe { libc::creat64("/tmp/test_result_fs\0".as_ptr() as *const libc::c_char, libc::S_IRUSR | libc::S_IWUSR) }
    // TODO: Close fd
}
fn test_fdopen(test_type: &str) -> i32 {
    let fd = unsafe { libc::open("/tmp/test_result_fs\0".as_ptr() as *const libc::c_char, libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC, libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IROTH as libc::mode_t) };
    let open_file = unsafe { libc::fdopen(fd, "w\x00".as_ptr() as *const libc::c_char) };
    let bytes_written: usize = unsafe { libc::fwrite("test\x00".as_ptr() as *mut libc::c_void, 1 as libc::size_t, 5 as libc::size_t, open_file) };
    if unsafe { libc::fclose(open_file) } == -1 {
        return -1
    };
    return bytes_written as i32
}
fn test_fopen(test_type: &str) -> i32 {
    return -1
}
fn test_fopen64(test_type: &str) -> i32 {
    return -1
}
fn test_open(test_type: &str) -> i32 {
    return -1
}
fn test_open64(test_type: &str) -> i32 {
    return -1
}
fn test_openat(test_type: &str) -> i32 {
    return -1
}
fn test_openat64(test_type: &str) -> i32 {
    return -1
}
fn test_chmod(test_type: &str) -> i32 {
    return -1
}
fn test_fchmod(test_type: &str) -> i32 {
    return -1
}
fn test_fchmodat(test_type: &str) -> i32 {
    return -1
}
fn test_chown(test_type: &str) -> i32 {
    return -1
}
fn test_lchown(test_type: &str) -> i32 {
    return -1
}
fn test_fchown(test_type: &str) -> i32 {
    return -1
}
fn test_fchownat(test_type: &str) -> i32 {
    return -1
}
fn test_link(test_type: &str) -> i32 {
    return -1
}
fn test_linkat(test_type: &str) -> i32 {
    return -1
}
fn test_symlink(test_type: &str) -> i32 {
    return -1
}
fn test_symlinkat(test_type: &str) -> i32 {
    return -1
}
fn test_rename(test_type: &str) -> i32 {
    return -1
}
fn test_renameat(test_type: &str) -> i32 {
    return -1
}
fn test_renameat2(test_type: &str) -> i32 {
    return -1
}
fn test_rmdir(test_type: &str) -> i32 {
    return -1
}
fn test_unlink(test_type: &str) -> i32 {
    return -1
}
fn test_unlinkat(test_type: &str) -> i32 {
    return -1
}
fn test_truncate(test_type: &str) -> i32 {
    return -1
}
fn test_ftruncate(test_type: &str) -> i32 {
    return -1
}
fn test_mknod(test_type: &str) -> i32 {
    return -1
}
fn test_mknodat(test_type: &str) -> i32 {
    return -1
}

#[derive(Debug)]
struct MetadataExtEq(Option<std::fs::Metadata>);

impl PartialEq for MetadataExtEq {
    fn eq(&self, other: &MetadataExtEq) -> bool {
        if (&self).0.is_some() && (&other).0.is_some() {
            let m1 = (&self).0.as_ref().unwrap();
            let m2 = (&other).0.as_ref().unwrap();
            return (m1.st_mode() == m2.st_mode())
                    && (m1.st_uid() == m2.st_uid()) && (m1.st_gid() == m2.st_gid())
                    && (m1.st_atime() == m2.st_atime()) && (m1.st_atime_nsec() == m2.st_atime_nsec())
                    && (m1.st_mtime() == m2.st_mtime()) && (m1.st_mtime_nsec() == m2.st_mtime_nsec())
                    && (m1.st_ctime() == m2.st_ctime()) && (m1.st_ctime_nsec() == m2.st_ctime_nsec())
        }
        false
    }
}

pub fn run_tests() {
    // TODO: Refactor to be similar to action framework, including positive and negative functions (compatability tests), benchmark tests, and vulnerability tests
    let tests: Vec<&str> = vec!["positive", "negative"];
    let modules: Vec<&str> = vec!["execl", "execle", "execlp", "execv", "execve", "execvp", "execvpe", "fexecve",
                                  "creat", "creat64", /*"fdopen", "fopen", "fopen64", "open", "open64", "openat", "openat64",
                                  "chmod", "fchmod", "fchmodat", "chown", "lchown", "fchown", "fchownat",
                                  "link", "linkat", "symlink", "symlinkat", "rename", "renameat", "renameat2",
                                  "rmdir", "unlink", "unlinkat", "truncate", "ftruncate", "mknod", "mknodat"*/];
    #[cfg(not(target_os = "linux"))]
    unimplemented!("WhiteBeam: Tests on non-Linux platforms are not currently supported");
    for module in modules.clone() {
        println!("WhiteBeam: Testing {}", module);
        let is_execution = module.contains("exec");
        if !(is_execution) {
            // Filesystem hook, create a test file
            let _e = std::fs::remove_file("/tmp/test_result_fs");
            std::fs::File::create("/tmp/test_result_fs").unwrap();
        }
        let original_metadata = MetadataExtEq(std::fs::metadata("/tmp/test_result_fs").ok());
        for test_type in tests.clone() {
            let exit_status_child = match module {
                "execv" => test_execv(test_type),
                "execve" => test_execve(test_type),
                "execvp" => test_execvp(test_type),
                "execvpe" => test_execvpe(test_type),
                "execl" => test_execl(test_type),
                "execle" => test_execle(test_type),
                "execlp" => test_execlp(test_type),
                "fexecve" => test_fexecve(test_type),
                "creat" => test_creat(test_type),
                "creat64" => test_creat64(test_type),
                "fdopen" => test_fdopen(test_type),
                "fopen" => test_fopen(test_type),
                "fopen64" => test_fopen64(test_type),
                "open" => test_open(test_type),
                "open64" => test_open64(test_type),
                "openat" => test_openat(test_type),
                "openat64" => test_openat64(test_type),
                "chmod" => test_chmod(test_type),
                "fchmod" => test_fchmod(test_type),
                "fchmodat" => test_fchmodat(test_type),
                "chown" => test_chown(test_type),
                "lchown" => test_lchown(test_type),
                "fchown" => test_fchown(test_type),
                "fchownat" => test_fchownat(test_type),
                "link" => test_link(test_type),
                "linkat" => test_linkat(test_type),
                "symlink" => test_symlink(test_type),
                "symlinkat" => test_symlinkat(test_type),
                "rename" => test_rename(test_type),
                "renameat" => test_renameat(test_type),
                "renameat2" => test_renameat2(test_type),
                "rmdir" => test_rmdir(test_type),
                "unlink" => test_unlink(test_type),
                "unlinkat" => test_unlinkat(test_type),
                "truncate" => test_truncate(test_type),
                "ftruncate" => test_ftruncate(test_type),
                "mknod" => test_mknod(test_type),
                "mknodat" => test_mknodat(test_type),
                _ => {eprintln!("WhiteBeam: No test found for {}", &module); -1}
            };
            match (test_type, is_execution) {
                ("positive", true) => {
                    // Positive Execution test
                    assert!(exit_status_child >= 0, "WhiteBeam: {} failed ({} test): exit code {}", module, test_type, exit_status_child);
                    let contents = std::fs::read_to_string("/tmp/test_result").expect("WhiteBeam: Could not read test result file");
                    assert_eq!(contents, format!("{}/target/release/libwhitebeam.so", env!("PWD")));
                    std::fs::remove_file("/tmp/test_result").expect("WhiteBeam: Failed to remove /tmp/test_result");
                },
                ("negative", true) => {
                    // Negative Execution test
                    // TODO: assert!(!exit_status_module.success());
                    assert_eq!(std::path::Path::new("/tmp/test_result").exists(), false);
                },
                ("positive", false) => {
                    // Positive Filesystem test
                    let new_metadata = MetadataExtEq(std::fs::metadata("/tmp/test_result_fs").ok());
                    assert_ne!(original_metadata, new_metadata);
                },
                ("negative", false) => {
                    // TODO: Negative Filesystem test
                    //let new_metadata = MetadataExtEq(std::fs::metadata("/tmp/test_result_fs").ok());
                    //assert_eq!(original_metadata, new_metadata);
                },
                _ => println!("WhiteBeam: Unknown test type")
            };
            println!("WhiteBeam: {} passed ({} test).", module, test_type);
        }
    }
    println!("WhiteBeam: All tests passed")
}
