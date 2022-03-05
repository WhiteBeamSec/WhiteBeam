whitebeam_test!("linux", interposition_execve {
    unsafe {
        let execve_symbol = libc::dlsym(libc::RTLD_DEFAULT, "execve\0".as_ptr() as *const libc::c_char);
        assert_ne!(execve_symbol, libc::execve as *mut libc::c_void);
    }
});

whitebeam_test!("linux", resolve_symbol_execve {
    // Resolves to libc::execve
    println!("Hello execve resolution test for Linux!");
});

whitebeam_test!("linux", resolve_symbol_dlopen {
    // Resolves to ldl
    println!("Hello dlopen resolution test for Linux!");
});