pub fn load_whitebeam() {
    // TODO: Cross platform
    let lib_path: std::path::PathBuf = std::path::PathBuf::from(format!("{}/target/release/libwhitebeam.so", env!("PWD")));
    assert!(lib_path.exists(), "WhiteBeam: libwhitebeam.so could not be found");
    let ld_audit: Option<std::ffi::OsString> = std::env::var_os("LD_AUDIT");
    if (ld_audit.is_none()) ||
       (ld_audit != Some(lib_path.as_os_str().to_os_string())) {
        // LD_AUDIT undefined. Restart program with LD_AUDIT set to libwhitebeam.so
        let test_path = unsafe { std::ffi::CStr::from_ptr(libc::getauxval(libc::AT_EXECFN) as *const u8)};
        let test_path_str = test_path.to_str().expect("Failed to convert test path to &str");
        let exit_status_test = std::process::Command::new(test_path_str)
            .stdout(std::process::Stdio::null())
            .env("LD_AUDIT", lib_path.as_os_str())
            .status().expect("Failed to execute process");
        assert!(exit_status_test.success());
    }
}

#[macro_export]
macro_rules! whitebeam_test {
    ($os:expr, $func:ident $body:block) => {
        #[test]
        #[cfg(target_os = $os)]
        fn $func() {
            crate::common::load_whitebeam();
            $body
        }
    };
}