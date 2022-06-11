// Tests basic interposition
whitebeam_test!("linux", interposition_00_execve {
    // execve() is hooked by WhiteBeam by default
    assert!(crate::common::is_hooked("execve"));
});

whitebeam_test!("linux", interposition_01_system {
    // system() is not hooked by WhiteBeam by default
    assert!(!(crate::common::is_hooked("system")));
});

// Tests live reloading of hooks
whitebeam_test!("linux", interposition_02_toggle_hook {
    // Waits up to ~100 milliseconds for dnotify signal to be delivered
    let mut enable_checks = 0;
    let mut disable_checks = 0;
    crate::common::toggle_hook("fexecve", true);
    while !(crate::common::is_hooked("fexecve")) {
        assert!(enable_checks < 3);
        enable_checks += 1;
        std::thread::sleep(std::time::Duration::from_millis(35));
    }
    crate::common::toggle_hook("fexecve", false);
    while crate::common::is_hooked("fexecve") {
        disable_checks += 1;
        if disable_checks < 3 {
            std::thread::sleep(std::time::Duration::from_millis(35));
        } else {
            break
        }
    }
    crate::common::toggle_hook("fexecve", true);
    assert!(disable_checks < 3);
});

// Tests generic hook
whitebeam_test!("linux", interposition_05_generic_hook_string {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_06_generic_hook_string_array {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_07_generic_hook_string_variadic {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_08_generic_hook_integer_signed {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_09_generic_hook_integer_signed_pointer {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_10_generic_hook_integer_unsigned {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_11_generic_hook_integer_unsigned_pointer {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_12_generic_hook_integer_unsigned_variadic {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_13_generic_hook_long_signed {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_14_generic_hook_long_unsigned {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_15_generic_hook_struct {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_16_generic_hook_struct_pointer {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_17_generic_hook_zero_args {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook zero arg test for Linux!");
});

whitebeam_test!("linux", interposition_18_generic_hook_six_args {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook six arg test for Linux!");
});

// Tests symbol resolution
whitebeam_test!("linux", interposition_19_resolve_symbol_execve {
    // Resolves to libc::execve
    println!("Hello execve resolution test for Linux!");
});

whitebeam_test!("linux", interposition_20_resolve_symbol_dlopen {
    // Resolves to ldl
    println!("Hello dlopen resolution test for Linux!");
});