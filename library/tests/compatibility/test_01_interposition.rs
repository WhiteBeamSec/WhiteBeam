whitebeam_test!("linux", interposition_00_execve {
    // execve() is hooked by WhiteBeam
    let execve_symbol = unsafe { libc::dlsym(libc::RTLD_DEFAULT, "execve\0".as_ptr() as *const libc::c_char) };
    let mut dl_info_execve = libc::Dl_info {
        dli_fname: core::ptr::null(),
        dli_fbase: core::ptr::null_mut(),
        dli_sname: core::ptr::null(),
        dli_saddr: core::ptr::null_mut(),
    };
    let execve_lib_name: *const libc::c_char = match unsafe { libc::dladdr(execve_symbol as *const libc::c_void, &mut dl_info_execve as *mut libc::Dl_info) } {
        0 => panic!("WhiteBeam: dladdr failed"),
        _ => dl_info_execve.dli_fname as *const libc::c_char
    };
    assert!(!(execve_lib_name.is_null()));
    let execve_lib_name_string = String::from(unsafe { std::ffi::CStr::from_ptr(execve_lib_name) }.to_str().expect("WhiteBeam: Unexpected null reference"));
    assert!(!(execve_lib_name_string.contains("libc.so")));
    assert!(execve_lib_name_string.contains("libwhitebeam.so"));
});

whitebeam_test!("linux", interposition_01_system {
    // system() is not hooked by WhiteBeam
    let system_symbol = unsafe { libc::dlsym(libc::RTLD_DEFAULT, "system\0".as_ptr() as *const libc::c_char) };
    let mut dl_info_system = libc::Dl_info {
        dli_fname: core::ptr::null(),
        dli_fbase: core::ptr::null_mut(),
        dli_sname: core::ptr::null(),
        dli_saddr: core::ptr::null_mut(),
    };
    let system_lib_name: *const libc::c_char = match unsafe { libc::dladdr(system_symbol as *const libc::c_void, &mut dl_info_system as *mut libc::Dl_info) } {
        0 => panic!("WhiteBeam: dladdr failed"),
        _ => dl_info_system.dli_fname as *const libc::c_char
    };
    assert!(!(system_lib_name.is_null()));
    let system_lib_name_string = String::from(unsafe { std::ffi::CStr::from_ptr(system_lib_name) }.to_str().expect("WhiteBeam: Unexpected null reference"));
    assert!(system_lib_name_string.contains("libc.so"));
    assert!(!(system_lib_name_string.contains("libwhitebeam.so")));
});

whitebeam_test!("linux", interposition_02_resolve_symbol_execve {
    // Resolves to libc::execve
    println!("Hello execve resolution test for Linux!");
});

whitebeam_test!("linux", interposition_03_resolve_symbol_dlopen {
    // Resolves to ldl
    println!("Hello dlopen resolution test for Linux!");
});