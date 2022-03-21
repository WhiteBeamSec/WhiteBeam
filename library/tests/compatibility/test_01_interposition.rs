whitebeam_test!("linux", interposition_00_execve {
    // execve() is hooked by WhiteBeam by default
    assert!(crate::common::is_hooked("execve"));
});

whitebeam_test!("linux", interposition_01_system {
    // system() is not hooked by WhiteBeam by default
    assert!(!(crate::common::is_hooked("system")));
});

whitebeam_test!("linux", interposition_02_toggle_hook_sanity {
    crate::common::toggle_hook("execve", true);
});

/*
TODO: Not functional yet, adding dnotify-based DB synchronization so a new process doesn't need to be spawned
whitebeam_test!("linux", interposition_03_enable_hook {
    match std::env::var("WB_ENABLE_HOOK") {
        Ok(_v) => {
            // In child
            assert!(crate::common::is_hooked("execve"));
        },
        Err(_e) => {
            // In parent
            crate::common::toggle_hook("execve", true);
            let test_path = unsafe { std::ffi::CStr::from_ptr(libc::getauxval(libc::AT_EXECFN) as *const u8)};
            let test_path_str = test_path.to_str().expect("Failed to convert test path to &str");
            let test_status = std::process::Command::new(test_path_str)
                .arg("interposition_04_disable_hook")
                .env("WB_ENABLE_HOOK", "1")
                .status().expect("WhiteBeam: Test command failed to start");
            assert!(test_status.success());
        }
    }
});

whitebeam_test!("linux", interposition_04_disable_hook {
    match std::env::var("WB_DISABLE_HOOK") {
        Ok(_v) => {
            // In child
            assert!(!(crate::common::is_hooked("execve")));
        },
        Err(_e) => {
            // In parent
            crate::common::toggle_hook("execve", false);
            let test_path = unsafe { std::ffi::CStr::from_ptr(libc::getauxval(libc::AT_EXECFN) as *const u8)};
            let test_path_str = test_path.to_str().expect("Failed to convert test path to &str");
            let test_status = std::process::Command::new(test_path_str)
                .arg("interposition_04_disable_hook")
                .env("WB_DISABLE_HOOK", "1")
                .status().expect("WhiteBeam: Test command failed to start");
            crate::common::toggle_hook("execve", true);
            assert!(test_status.success());
        }
    }
});
*/

whitebeam_test!("linux", interposition_05_resolve_symbol_execve {
    // Resolves to libc::execve
    println!("Hello execve resolution test for Linux!");
});

whitebeam_test!("linux", interposition_06_resolve_symbol_dlopen {
    // Resolves to ldl
    println!("Hello dlopen resolution test for Linux!");
});