pub fn load_whitebeam(test: &str) -> bool {
    // TODO: Cross platform
    let lib_path: std::path::PathBuf = std::path::PathBuf::from(format!("{}/target/release/libwhitebeam.so", env!("PWD")));
    assert!(lib_path.exists(), "WhiteBeam: libwhitebeam.so could not be found");
    let ld_audit: Option<std::ffi::OsString> = std::env::var_os("LD_AUDIT");
    // TODO: Check zeroth index of colon separated variable instead of checking if LD_AUDIT equals the library path
    if (ld_audit.is_none()) ||
       (ld_audit != Some(lib_path.as_os_str().to_os_string())) {
        // LD_AUDIT undefined. Restart program with LD_PRELOAD set to libwhitebeam.so
        let test_path = unsafe { std::ffi::CStr::from_ptr(libc::getauxval(libc::AT_EXECFN) as *const u8)};
        let test_path_str = test_path.to_str().expect("Failed to convert test path to &str");
        let exit_status_test = std::process::Command::new(test_path_str)
            .args(&["--test", test])
            // TODO: Gate behind verbose flag
            .stdout(std::process::Stdio::null())
            // Set LD_PRELOAD to test initialization of LD_AUDIT (/etc/ld.so.preload behavior)
            .env("LD_PRELOAD", lib_path.as_os_str())
            .status().expect("Failed to execute process");
        assert!(exit_status_test.success());
        return true;
    }
    return false;
}

#[macro_export]
macro_rules! whitebeam_test {
    ($os:expr, $func:ident $body:block) => {
        #[test]
        #[cfg(target_os = $os)]
        fn $func() {
            let in_parent_process: bool = crate::common::load_whitebeam(stringify!($func));
            if in_parent_process {
                return;
            }
            $body
        }
    };
}