#[macro_use]
mod common;

macro_rules! whitebeam_test {
    ($os:expr, $func:ident $body:block) => {
        #[cfg(target_os = $os)]
        #[allow(non_snake_case)]
        #[linkme::distributed_slice(crate::TEST_INDEX)]
        fn $func() {
            print!("Test {}", stringify!($func));
            $body
            println!("... \x1b[1;32mok\x1b[0m");
        }
    };
}

mod compatibility {
    automod::dir!("tests/compatibility");
}

mod vulnerability {
    automod::dir!("tests/vulnerability");
}

// Collect tests in distributed slice
#[linkme::distributed_slice]
pub static TEST_INDEX: [fn()] = [..];

fn main() {
    let lib_path: std::path::PathBuf = std::path::PathBuf::from(format!("{}/target/release/libwhitebeam.so", env!("PWD")));
    assert!(lib_path.exists(), "WhiteBeam: libwhitebeam.so could not be found");
    let ld_audit: Option<std::ffi::OsString> = std::env::var_os("LD_AUDIT");
    // TODO: Check zeroth index of colon separated variable instead of checking if LD_AUDIT equals the library path
    if (ld_audit.is_none()) ||
       (ld_audit != Some(lib_path.as_os_str().to_os_string())) {
        // LD_AUDIT undefined. Restart program with LD_PRELOAD set to libwhitebeam.so
        let test_path = unsafe { std::ffi::CStr::from_ptr(libc::getauxval(libc::AT_EXECFN) as *const libc::c_char)};
        let test_path_str = test_path.to_str().expect("Failed to convert test path to &str");
        let exit_status_test = std::process::Command::new(test_path_str)
            // Set LD_PRELOAD to test initialization of LD_AUDIT (/etc/ld.so.preload behavior)
            .env("LD_PRELOAD", lib_path.as_os_str())
            .status().expect("Failed to execute process");
        assert!(exit_status_test.success());
        return;
    }
    // TODO: Allow a specific test to be run
    println!("WhiteBeam: Running tests");
    for test in TEST_INDEX.iter().rev() {
        test();
    }
}