use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

pub fn osstr_split_at_byte(osstr_input: &std::ffi::OsStr, byte: u8) -> (&OsStr, &OsStr) {
    for (i, b) in osstr_input.as_bytes().iter().enumerate() {
        if b == &byte {
            return (std::ffi::OsStr::from_bytes(&osstr_input.as_bytes()[..i]),
                std::ffi::OsStr::from_bytes(&osstr_input.as_bytes()[i + 1..]));
        }
    }
    (&*osstr_input, std::ffi::OsStr::from_bytes(&osstr_input.as_bytes()[osstr_input.len()..osstr_input.len()]))
}

whitebeam_test!("linux", initialization_00_ld_audit {
    let rtld_audit_lib_path = std::path::PathBuf::from(format!("{}/target/release/libwhitebeam.so", env!("PWD")));
    let ld_audit: Option<std::ffi::OsString> = std::env::var_os("LD_AUDIT");
    assert!(ld_audit.is_some());
    assert!(osstr_split_at_byte(&ld_audit.expect("WhiteBeam: Unexpected null reference"), b':').0 == rtld_audit_lib_path)
});

whitebeam_test!("linux", initialization_01_ld_bind_not {
    let ld_bind_not_value: std::ffi::OsString = std::ffi::OsString::from("1");
    let ld_bind_not: Option<std::ffi::OsString> = std::env::var_os("LD_BIND_NOT");
    assert!(ld_bind_not.is_some());
    assert!(ld_bind_not.expect("WhiteBeam: Unexpected null reference") == ld_bind_not_value)
});

whitebeam_test!("linux", initialization_02_env_sanity {
    let shell_value = std::process::Command::new("/bin/bash").arg("-c").arg("/usr/bin/printenv SHELL").output().expect("bash command failed to start");
    assert!(shell_value.stdout.iter().eq(b"/bin/bash\n"));
});

whitebeam_test!("linux", initialization_03_wb_parent {
    // /proc/self/environ preserves environment variables after they are unset at runtime
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execve("/bin/bash\0".as_ptr() as *const libc::c_char,
                              ["/bin/bash\0".as_ptr() as *const libc::c_char, "-c\0".as_ptr() as *const libc::c_char, "/usr/bin/grep -qPa 'WB_PARENT=/bin/bash\\0' /proc/self/environ\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(),
                              std::ptr::null()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
    }
});

whitebeam_test!("linux", initialization_04_wb_prog {
    // /proc/self/environ preserves environment variables after they are unset at runtime
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execve("/bin/bash\0".as_ptr() as *const libc::c_char,
                              ["/bin/bash\0".as_ptr() as *const libc::c_char, "-c\0".as_ptr() as *const libc::c_char, "/usr/bin/grep -qPa 'WB_PROG=/usr/bin/grep\\0' /proc/self/environ\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(),
                              std::ptr::null()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
    }
});

pub unsafe fn gnu_get_libc_version() -> *const libc::c_char {
    extern "C" {
        fn gnu_get_libc_version() -> *const libc::c_char;
    }
    gnu_get_libc_version()
}

whitebeam_test!("linux", initialization_05_stable_ld_audit {
    let libc_version = unsafe { gnu_get_libc_version() };
    let libc_version_str = unsafe { std::ffi::CStr::from_ptr(libc_version).to_str().expect("WhiteBeam: Failed to determine libc version") };
    let libc_version_split: Vec<u32> = libc_version_str.split('.').map(|n| n.parse::<u32>().expect("WhiteBeam: Failed to parse libc version")).collect::<Vec<u32>>();
    assert!(libc_version_split.len() >= 2, "WhiteBeam: Failed to parse libc version");
    let libc_version_major = libc_version_split[0];
    let libc_version_minor = libc_version_split[1];
    assert!(((libc_version_major > 2) || ((libc_version_major == 2) && (libc_version_minor >= 35))), "WhiteBeam: libc version 2.35 or higher required");
});