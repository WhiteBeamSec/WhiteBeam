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
                              ["/usr/bin/grep\0".as_ptr() as *const libc::c_char, "libwhitebeam.so\0".as_ptr() as *const libc::c_char, "/proc/self/maps\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(),
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

// Tests for:
// execle
// execlp
// execv
// execvp
// execvpe
// fexecve
// posix_spawn
// posix_spawnp
// dlopen
// dlmopen
// kill