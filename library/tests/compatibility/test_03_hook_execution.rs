// TODO: Tests to ensure environment is not corrupted
// TODO: dlerror() tests with Prevention

whitebeam_test!("linux", execution_00_execve_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execve("/usr/bin/touch\0".as_ptr() as *const libc::c_char,
                              ["/usr/bin/touch\0".as_ptr() as *const libc::c_char, "/tmp/execve_test\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(),
                              std::ptr::null()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/execve_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_01_execve_library_loaded {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execve("/usr/bin/grep\0".as_ptr() as *const libc::c_char,
                              ["/usr/bin/grep\0".as_ptr() as *const libc::c_char, "-q\0".as_ptr() as *const libc::c_char, "libwhitebeam.so\0".as_ptr() as *const libc::c_char, "/proc/self/maps\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(),
                              std::ptr::null()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
    }
});

whitebeam_test!("linux", execution_02_execl_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execl("/usr/bin/touch\0".as_ptr() as *const libc::c_char,
                             "/usr/bin/touch\0".as_ptr() as *const libc::c_char, "/tmp/execl_test\0".as_ptr() as *const libc::c_char, std::ptr::null() as *const libc::c_char); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/execl_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_03_execle_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execle("/usr/bin/touch\0".as_ptr() as *const libc::c_char,
                              "/usr/bin/touch\0".as_ptr() as *const libc::c_char, "/tmp/execle_test\0".as_ptr() as *const libc::c_char, std::ptr::null() as *const libc::c_char,
                              std::ptr::null() as *const libc::c_char); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/execle_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_04_execlp_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execlp("touch\0".as_ptr() as *const libc::c_char,
                              "touch\0".as_ptr() as *const libc::c_char, "/tmp/execlp_test\0".as_ptr() as *const libc::c_char, std::ptr::null() as *const libc::c_char); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/execlp_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_05_execv_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execv("/usr/bin/touch\0".as_ptr() as *const libc::c_char,
                             ["/usr/bin/touch\0".as_ptr() as *const libc::c_char, "/tmp/execv_test\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/execv_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_06_execvp_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execvp("touch\0".as_ptr() as *const libc::c_char,
                              ["touch\0".as_ptr() as *const libc::c_char, "/tmp/execvp_test\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/execvp_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_07_execvpe_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execvpe("touch\0".as_ptr() as *const libc::c_char,
                               ["touch\0".as_ptr() as *const libc::c_char, "/tmp/execvpe_test\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(),
                               std::ptr::null()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/execvpe_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_08_fexecve_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        let fd: libc::c_int = unsafe { libc::open("/usr/bin/touch\0".as_ptr() as *const libc::c_char, libc::O_RDONLY) };
        unsafe { libc::fexecve(fd, ["/usr/bin/touch\0".as_ptr() as *const libc::c_char, "/tmp/fexecve_test\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(), std::ptr::null()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/fexecve_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_09_posix_spawn_simple {
    let mut pid: libc::pid_t = 0;
    unsafe { libc::posix_spawn(&mut pid as *mut libc::pid_t,
                               "/usr/bin/touch\0".as_ptr() as *const libc::c_char,
                               std::ptr::null(),
                               std::ptr::null(),
                               ["/usr/bin/touch\0".as_ptr() as *const libc::c_char, "/tmp/posix_spawn_test\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr() as *const *mut libc::c_char,
                               std::ptr::null()); }
    let mut status = 0;
    unsafe { libc::waitpid(pid, &mut status, 0); }
    assert!(status == 0);
    let test_path = std::path::Path::new("/tmp/posix_spawn_test");
    assert!(test_path.exists());
    std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
});

whitebeam_test!("linux", execution_10_posix_spawnp_simple {
    let mut pid: libc::pid_t = 0;
    unsafe { libc::posix_spawnp(&mut pid as *mut libc::pid_t,
                                "touch\0".as_ptr() as *const libc::c_char,
                                std::ptr::null(),
                                std::ptr::null(),
                                ["touch\0".as_ptr() as *const libc::c_char, "/tmp/posix_spawnp_test\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr() as *const *mut libc::c_char,
                                std::ptr::null()); }
    let mut status = 0;
    unsafe { libc::waitpid(pid, &mut status, 0); }
    assert!(status == 0);
    let test_path = std::path::Path::new("/tmp/posix_spawnp_test");
    assert!(test_path.exists());
    std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
});

whitebeam_test!("linux", execution_11_kill_simple {
    let mut pid: libc::pid_t = 0;
    unsafe { libc::posix_spawn(&mut pid as *mut libc::pid_t,
                               "/usr/bin/sleep\0".as_ptr() as *const libc::c_char,
                               std::ptr::null(),
                               std::ptr::null(),
                               ["/usr/bin/sleep\0".as_ptr() as *const libc::c_char, "10\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr() as *const *mut libc::c_char,
                               std::ptr::null()); }
    unsafe { libc::kill(pid, libc::SIGKILL); }
    let mut status = 0;
    unsafe { libc::waitpid(pid, &mut status, 0); }
    assert!(status == libc::SIGKILL as i32);
});

whitebeam_test!("linux", execution_12_dlopen_lazy {
    let handle = unsafe { libc::dlopen("libc.so.6\0".as_ptr() as *const libc::c_char, libc::RTLD_LAZY) };
    assert!(handle != std::ptr::null_mut());
    unsafe { libc::dlclose(handle); }
});

whitebeam_test!("linux", execution_13_dlopen_now {
    let handle = unsafe { libc::dlopen("libm.so.6\0".as_ptr() as *const libc::c_char, libc::RTLD_NOW) };
    assert!(handle != std::ptr::null_mut());
    unsafe { libc::dlclose(handle); }
});

whitebeam_test!("linux", execution_14_dlopen_absolute_path {
    let libc: &str = &format!("/lib/{}-linux-gnu/libc.so.6\0", std::env::consts::ARCH);
    let handle = unsafe { libc::dlopen(libc.as_ptr() as *const libc::c_char, libc::RTLD_LAZY) };
    assert!(handle != std::ptr::null_mut());
    unsafe { libc::dlclose(handle); }
});

whitebeam_test!("linux", execution_15_dlopen_null {
    let handle = unsafe { libc::dlopen(std::ptr::null() as *const libc::c_char, libc::RTLD_LAZY) };
    assert!(handle != std::ptr::null_mut());
    unsafe { libc::dlclose(handle); }
});

whitebeam_test!("linux", execution_16_dlopen_lazy_call_unhooked {
    let handle = unsafe { libc::dlopen("libc.so.6\0".as_ptr() as *const libc::c_char, libc::RTLD_LAZY) };
    assert!(handle != std::ptr::null_mut());
    let getpid_ptr = unsafe { libc::dlsym(handle, "getpid\0".as_ptr() as *const libc::c_char) };
    assert!(getpid_ptr != std::ptr::null_mut());
    let getpid_fn: unsafe extern "C" fn() -> libc::pid_t = unsafe { std::mem::transmute(getpid_ptr) };
    assert!(unsafe { getpid_fn() } == unsafe { libc::getpid() });
    unsafe { libc::dlclose(handle); }
});

whitebeam_test!("linux", execution_17_dlopen_lazy_call_hooked {
    let handle = unsafe { libc::dlopen("libc.so.6\0".as_ptr() as *const libc::c_char, libc::RTLD_LAZY) };
    assert!(handle != std::ptr::null_mut());
    let kill_ptr = unsafe { libc::dlsym(handle, "kill\0".as_ptr() as *const libc::c_char) };
    assert!(kill_ptr != std::ptr::null_mut());
    let kill_fn: unsafe extern "C" fn(libc::pid_t, libc::c_int) -> libc::c_int = unsafe { std::mem::transmute(kill_ptr) };
    assert!(unsafe { kill_fn(libc::getpid(), libc::SIGCONT) } == 0);
    unsafe { libc::dlclose(handle); }
});

whitebeam_test!("linux", execution_18_dlopen_now_call_unhooked {
    let handle = unsafe { libc::dlopen("libc.so.6\0".as_ptr() as *const libc::c_char, libc::RTLD_NOW) };
    assert!(handle != std::ptr::null_mut());
    let getpid_ptr = unsafe { libc::dlsym(handle, "getpid\0".as_ptr() as *const libc::c_char) };
    assert!(getpid_ptr != std::ptr::null_mut());
    let getpid_fn: unsafe extern "C" fn() -> libc::pid_t = unsafe { std::mem::transmute(getpid_ptr) };
    assert!(unsafe { getpid_fn() } == unsafe { libc::getpid() });
    unsafe { libc::dlclose(handle); }
});

whitebeam_test!("linux", execution_19_dlopen_now_call_hooked {
    let handle = unsafe { libc::dlopen("libc.so.6\0".as_ptr() as *const libc::c_char, libc::RTLD_NOW) };
    assert!(handle != std::ptr::null_mut());
    let kill_ptr = unsafe { libc::dlsym(handle, "kill\0".as_ptr() as *const libc::c_char) };
    assert!(kill_ptr != std::ptr::null_mut());
    let kill_fn: unsafe extern "C" fn(libc::pid_t, libc::c_int) -> libc::c_int = unsafe { std::mem::transmute(kill_ptr) };
    assert!(unsafe { kill_fn(libc::getpid(), libc::SIGCONT) } == 0);
    unsafe { libc::dlclose(handle); }
});

whitebeam_test!("linux", execution_20_dlerror_not_found {
    let handle = unsafe { libc::dlopen("missing.so\0".as_ptr() as *const libc::c_char, libc::RTLD_LAZY) };
    assert!(handle == std::ptr::null_mut());
    let error: *const libc::c_char = unsafe { libc::dlerror() };
    let error_str = unsafe { std::ffi::CStr::from_ptr(error).to_str().expect("WhiteBeam: Failed to convert dlerror to &str type") };
    assert!(error_str == "missing.so: cannot open shared object file: No such file or directory");
});

whitebeam_test!("linux", execution_21_dlopen_noload {
    let handle = unsafe { libc::dlopen("libcap.so.2\0".as_ptr() as *const libc::c_char, libc::RTLD_LAZY | libc::RTLD_NOLOAD) };
    assert!(handle == std::ptr::null_mut());
});

whitebeam_test!("linux", execution_22_dlmopen_base {
    let handle = unsafe { libc::dlmopen(libc::LM_ID_BASE, "libcap.so.2\0".as_ptr() as *const libc::c_char, libc::RTLD_LAZY) };
    assert!(handle != std::ptr::null_mut());
    unsafe { libc::dlclose(handle); }
});

whitebeam_test!("linux", execution_23_dlmopen_absolute_path {
    let libcap: &str = &format!("/lib/{}-linux-gnu/libcap.so.2\0", std::env::consts::ARCH);
    let handle = unsafe { libc::dlmopen(libc::LM_ID_BASE, libcap.as_ptr() as *const libc::c_char, libc::RTLD_LAZY) };
    assert!(handle != std::ptr::null_mut());
    unsafe { libc::dlclose(handle); }
});

whitebeam_test!("linux", execution_24_dlmopen_lazy_call_unhooked {
    let handle = unsafe { libc::dlmopen(libc::LM_ID_BASE, "libcap.so.2\0".as_ptr() as *const libc::c_char, libc::RTLD_LAZY) };
    assert!(handle != std::ptr::null_mut());
    let cap_from_name_ptr = unsafe { libc::dlsym(handle, "cap_from_name\0".as_ptr() as *const libc::c_char) };
    assert!(cap_from_name_ptr != std::ptr::null_mut());
    let mut cap_value: libc::c_int = 0;
    let cap_from_name_fn: unsafe extern "C" fn(*const libc::c_char, *mut libc::c_int) -> libc::c_int = unsafe { std::mem::transmute(cap_from_name_ptr) };
    assert!(unsafe { cap_from_name_fn("CAP_SETUID\0".as_ptr() as *const libc::c_char, &mut cap_value as *mut _) } == 0);
    assert!(cap_value == 7);
    unsafe { libc::dlclose(handle); }
});

whitebeam_test!("linux", execution_25_execve_prevention {
    crate::common::toggle_prevention(true);
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execve("/bin/bash\0".as_ptr() as *const libc::c_char,
                              ["/bin/bash\0".as_ptr() as *const libc::c_char, "-c\0".as_ptr() as *const libc::c_char, "/usr/bin/touch /tmp/execve_prevention_test 2>/dev/null\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(),
                              std::ptr::null()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        crate::common::toggle_prevention(false);
        assert!(status != 0);
        let test_path = std::path::Path::new("/tmp/execve_prevention_test");
        assert!(!(test_path.exists()));
    }
});

whitebeam_test!("linux", execution_26_execve_prevention_allowed {
    crate::common::toggle_prevention(true);
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execve("/bin/bash\0".as_ptr() as *const libc::c_char,
                              ["/bin/bash\0".as_ptr() as *const libc::c_char, "-c\0".as_ptr() as *const libc::c_char, "/usr/bin/whoami > /dev/null\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(),
                              std::ptr::null()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        crate::common::toggle_prevention(false);
        assert!(status == 0);
    }
});

whitebeam_test!("linux", execution_27_execve_prevention_errno_not_found {
    crate::common::toggle_prevention(true);
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        // Needed to simulate /etc/ld.so.preload
        let ld_preload_var = format!("LD_PRELOAD={}/target/release/libwhitebeam.so\0", env!("PWD"));
        unsafe { libc::execve("/bin/bash\0".as_ptr() as *const libc::c_char,
                              ["/bin/bash\0".as_ptr() as *const libc::c_char, "-c\0".as_ptr() as *const libc::c_char, "exec missingbinary 2>/dev/null\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(),
                              [ld_preload_var.as_ptr() as *const libc::c_char, "WB_PROG=/bin/bash\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        crate::common::toggle_prevention(false);
        assert!((status >> 8) == 127);
    }
});

whitebeam_test!("linux", execution_28_execve_prevention_errno_not_permitted {
    crate::common::toggle_prevention(true);
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        // Needed to simulate /etc/ld.so.preload
        let ld_preload_var = format!("LD_PRELOAD={}/target/release/libwhitebeam.so\0", env!("PWD"));
        unsafe { libc::execve("/bin/bash\0".as_ptr() as *const libc::c_char,
                              ["/bin/bash\0".as_ptr() as *const libc::c_char, "-c\0".as_ptr() as *const libc::c_char, "exec id 2>/dev/null\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(),
                              [ld_preload_var.as_ptr() as *const libc::c_char, "WB_PROG=/bin/bash\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        crate::common::toggle_prevention(false);
        assert!((status >> 8) == 126);
    }
});

whitebeam_test!("linux", execution_29_fexecve_prevention {
    crate::common::toggle_prevention(true);
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        let bash_fd: libc::c_int = unsafe { libc::open("/bin/bash\0".as_ptr() as *const libc::c_char, libc::O_RDONLY) };
        unsafe { libc::fexecve(bash_fd,
                              ["/bin/bash\0".as_ptr() as *const libc::c_char, "-c\0".as_ptr() as *const libc::c_char, "/usr/bin/touch /tmp/fexecve_prevention_test 2>/dev/null\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(),
                              std::ptr::null()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        crate::common::toggle_prevention(false);
        assert!(status != 0);
        let test_path = std::path::Path::new("/tmp/fexecve_prevention_test");
        assert!(!(test_path.exists()));
    }
});

whitebeam_test!("linux", execution_30_posix_spawn_prevention {
    crate::common::toggle_prevention(true);
    let mut pid: libc::pid_t = 0;
    unsafe { libc::posix_spawn(&mut pid as *mut libc::pid_t,
                               "/bin/bash\0".as_ptr() as *const libc::c_char,
                               std::ptr::null(),
                               std::ptr::null(),
                               ["/bin/bash\0".as_ptr() as *const libc::c_char, "-c\0".as_ptr() as *const libc::c_char, "/usr/bin/touch /tmp/posix_spawn_prevention_test 2>/dev/null\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr() as *const *mut libc::c_char,
                               std::ptr::null()); }
    let mut status = 0;
    unsafe { libc::waitpid(pid, &mut status, 0); }
    crate::common::toggle_prevention(false);
    assert!(status != 0);
    let test_path = std::path::Path::new("/tmp/posix_spawn_prevention_test");
    assert!(!(test_path.exists()));
});

whitebeam_test!("linux", execution_31_dlopen_prevention {
    crate::common::toggle_prevention(true);
    // Mute stderr
    let dup_stderr_fd = unsafe { libc::dup(libc::STDERR_FILENO) };
    let dev_null_fd = unsafe { libc::open("/dev/null\0".as_ptr() as *const libc::c_char, libc::O_RDWR) };
    unsafe { libc::dup2(dev_null_fd, libc::STDERR_FILENO); }
    let handle = unsafe { libc::dlopen("libcap.so.2\0".as_ptr() as *const libc::c_char, libc::RTLD_LAZY) };
    // Restore stderr
    unsafe { libc::dup2(dup_stderr_fd, libc::STDERR_FILENO); }
    unsafe { libc::close(dup_stderr_fd); }
    crate::common::toggle_prevention(false);
    assert!(handle == std::ptr::null_mut());
});

whitebeam_test!("linux", execution_32_dlopen_prevention_allowed {
    crate::common::toggle_prevention(true);
    let handle = unsafe { libc::dlopen("libm.so.6\0".as_ptr() as *const libc::c_char, libc::RTLD_LAZY) };
    crate::common::toggle_prevention(false);
    assert!(handle != std::ptr::null_mut());
});

whitebeam_test!("linux", execution_33_dlmopen_prevention {
    crate::common::toggle_prevention(true);
    // Mute stderr
    let dup_stderr_fd = unsafe { libc::dup(libc::STDERR_FILENO) };
    let dev_null_fd = unsafe { libc::open("/dev/null\0".as_ptr() as *const libc::c_char, libc::O_RDWR) };
    unsafe { libc::dup2(dev_null_fd, libc::STDERR_FILENO); }
    let handle = unsafe { libc::dlmopen(libc::LM_ID_BASE, "libcap.so.2\0".as_ptr() as *const libc::c_char, libc::RTLD_LAZY) };
    // Restore stderr
    unsafe { libc::dup2(dup_stderr_fd, libc::STDERR_FILENO); }
    unsafe { libc::close(dup_stderr_fd); }
    crate::common::toggle_prevention(false);
    assert!(handle == std::ptr::null_mut());
});

whitebeam_test!("linux", execution_34_dlmopen_prevention_allowed {
    crate::common::toggle_prevention(true);
    let handle = unsafe { libc::dlmopen(libc::LM_ID_BASE, "libm.so.6\0".as_ptr() as *const libc::c_char, libc::RTLD_LAZY) };
    crate::common::toggle_prevention(false);
    assert!(handle != std::ptr::null_mut());
});