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

whitebeam_test!("linux", interposition_03_enable_hook {
    std::thread::sleep(std::time::Duration::from_millis(1000));
    crate::common::toggle_hook("execve", true);
    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(crate::common::is_hooked("execve"));
});

whitebeam_test!("linux", interposition_04_disable_hook {
    std::thread::sleep(std::time::Duration::from_millis(1000));
    crate::common::toggle_hook("execve", false);
    std::thread::sleep(std::time::Duration::from_millis(500));
    let execve_hooked = crate::common::is_hooked("execve");
    crate::common::toggle_hook("execve", true);
    assert!(!(execve_hooked));
});

whitebeam_test!("linux", interposition_05_resolve_symbol_execve {
    // Resolves to libc::execve
    println!("Hello execve resolution test for Linux!");
});

whitebeam_test!("linux", interposition_06_resolve_symbol_dlopen {
    // Resolves to ldl
    println!("Hello dlopen resolution test for Linux!");
});