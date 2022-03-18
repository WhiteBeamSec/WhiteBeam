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
    let status = std::process::Command::new("/bin/bash").arg("-c").arg("/usr/bin/grep -Pa 'WB_PARENT=/bin/bash\\0' /proc/self/environ").status().expect("bash command failed to start");
    assert!(status.success());
});

whitebeam_test!("linux", initialization_04_wb_prog {
    // /proc/self/environ preserves environment variables after they are unset at runtime
    let status = std::process::Command::new("/bin/bash").arg("-c").arg("/usr/bin/grep -Pa 'WB_PROG=/usr/bin/grep\\0' /proc/self/environ").status().expect("bash command failed to start");
    assert!(status.success());
});