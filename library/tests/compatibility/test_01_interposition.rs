// Tests basic interposition
whitebeam_test!("linux", interposition_00_execve {
    // execve() is hooked by WhiteBeam by default
    let libc: String = if std::path::PathBuf::from("/lib64").exists() { String::from("/lib64/libc.so.6") }
                       else { format!("/lib/{}-linux-gnu/libc.so.6", std::env::consts::ARCH) };
    assert!(crate::common::is_hooked(&libc, "execve"));
});

whitebeam_test!("linux", interposition_01_mmap {
    // mmap() is not hooked by WhiteBeam by default
    let libc: String = if std::path::PathBuf::from("/lib64").exists() { String::from("/lib64/libc.so.6") }
                       else { format!("/lib/{}-linux-gnu/libc.so.6", std::env::consts::ARCH) };
    assert!(!(crate::common::is_hooked(&libc, "mmap")));
});

// Tests live reloading of hooks
whitebeam_test!("linux", interposition_02_toggle_hook {
    // Waits up to ~100 milliseconds for dnotify signal to be delivered
    let mut enable_checks = 0;
    let mut disable_checks = 0;
    let libc: String = if std::path::PathBuf::from("/lib64").exists() { String::from("/lib64/libc.so.6") }
                       else { format!("/lib/{}-linux-gnu/libc.so.6", std::env::consts::ARCH) };
    let sql_create = r#"BEGIN;
                        INSERT OR IGNORE INTO Hook (symbol, library, enabled, language, class)
                        SELECT * FROM (VALUES ("test_hook", (SELECT value FROM Setting WHERE param="SystemLibraryPath") || "libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Test")));
                        COMMIT;"#;
    crate::common::load_sql(sql_create);
    crate::common::toggle_hook("test_hook", true);
    while !(crate::common::is_hooked(&libc, "test_hook")) {
        assert!(enable_checks < 3);
        enable_checks += 1;
        std::thread::sleep(std::time::Duration::from_millis(35));
    }
    crate::common::toggle_hook("test_hook", false);
    while crate::common::is_hooked(&libc, "test_hook") {
        disable_checks += 1;
        if disable_checks < 3 {
            std::thread::sleep(std::time::Duration::from_millis(35));
        } else {
            break
        }
    }
    let sql_delete = r#"BEGIN;
                        DELETE FROM Hook WHERE symbol = "test_hook" AND library = (SELECT value FROM Setting WHERE param="SystemLibraryPath") || "libc.so.6";
                        COMMIT;"#;
    crate::common::load_sql(sql_delete);
    assert!(disable_checks < 3);
});

// Tests generic hook
whitebeam_test!("linux", interposition_03_generic_hook_string {
    let libc: String = if std::path::PathBuf::from("/lib64").exists() { String::from("/lib64/libc.so.6") }
                       else { format!("/lib/{}-linux-gnu/libc.so.6", std::env::consts::ARCH) };
    // Load strdup hook
    let sql_create = r#"BEGIN;
                        INSERT OR IGNORE INTO Hook (symbol, library, enabled, language, class)
                        SELECT * FROM (VALUES ("strdup", (SELECT value FROM Setting WHERE param="SystemLibraryPath") || "libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Test")));
                        INSERT OR IGNORE INTO Argument (name, position, hook, datatype)
                        SELECT * FROM (VALUES ("s", 0, (SELECT id FROM Hook WHERE library = (SELECT value FROM Setting WHERE param="SystemLibraryPath") || "libc.so.6" AND symbol="strdup"), (SELECT id FROM Datatype WHERE datatype="String")));
                        COMMIT;"#;
    crate::common::load_sql(sql_create);
    // Waits up to ~100 milliseconds for dnotify signal to be delivered
    let mut enable_checks = 0;
    while !(crate::common::is_hooked(&libc, "strdup")) {
        assert!(enable_checks < 3);
        enable_checks += 1;
        std::thread::sleep(std::time::Duration::from_millis(35));
    }
    let generic_hook_addr: usize = unsafe { libc::dlsym(libc::RTLD_DEFAULT, "strdup\0".as_ptr() as *const libc::c_char) } as usize;
    assert!(generic_hook_addr != 0);
    let hooked_strdup: unsafe extern "C" fn(arg1: *const libc::c_char, args: ...) -> isize = unsafe { std::mem::transmute(generic_hook_addr) };
    let orig_cstring = "test\0".as_ptr() as *const libc::c_char;
    let dup_cstring = unsafe { hooked_strdup(orig_cstring) } as *const libc::c_char;
    // Clean up
    let sql_delete = r#"BEGIN;
                        DELETE FROM Argument WHERE hook=(SELECT id FROM Hook WHERE symbol = "strdup"  AND library = (SELECT value FROM Setting WHERE param="SystemLibraryPath") || "libc.so.6");
                        DELETE FROM Hook WHERE symbol = "strdup" AND library = (SELECT value FROM Setting WHERE param="SystemLibraryPath") || "libc.so.6";
                        COMMIT;"#;
    crate::common::load_sql(sql_delete);
    assert!(!(dup_cstring.is_null()));
    assert!(unsafe { libc::strncmp(orig_cstring, dup_cstring, libc::strlen(orig_cstring)) } == 0);
});

/*
whitebeam_test!("linux", interposition_04_generic_hook_string_array {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_05_generic_hook_string_variadic {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_06_generic_hook_integer_signed {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_07_generic_hook_integer_signed_pointer {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_08_generic_hook_integer_unsigned {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_09_generic_hook_integer_unsigned_pointer {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_10_generic_hook_integer_unsigned_variadic {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_11_generic_hook_long_signed {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_12_generic_hook_long_unsigned {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_13_generic_hook_struct {
    // TODO: Load inet_ntoa hook with whitebeam command
    /*
    The structure in_addr as used in inet_ntoa() (..) is defined in <netinet/in.h> as:
        typedef uint32_t in_addr_t;
        struct in_addr {
            in_addr_t s_addr;
        };
    */
    println!("Hello generic hook test for Linux!");
});

whitebeam_test!("linux", interposition_14_generic_hook_struct_pointer {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook test for Linux!");
});
*/

whitebeam_test!("linux", interposition_15_generic_hook_zero_args {
    let libc: String = if std::path::PathBuf::from("/lib64").exists() { String::from("/lib64/libc.so.6") }
                       else { format!("/lib/{}-linux-gnu/libc.so.6", std::env::consts::ARCH) };
    let getlogin_result_unhooked = unsafe { libc::getlogin() } as *mut libc::c_char;
    assert!(!(getlogin_result_unhooked.is_null()));
    let sql_create = r#"BEGIN;
                        INSERT OR IGNORE INTO Hook (symbol, library, enabled, language, class)
                        SELECT * FROM (VALUES ("getlogin", (SELECT value FROM Setting WHERE param="SystemLibraryPath") || "libc.so.6", 1, (SELECT id FROM HookLanguage WHERE language="C"), (SELECT id FROM HookClass WHERE class="Test")));
                        COMMIT;"#;
    crate::common::load_sql(sql_create);
    // Waits up to ~100 milliseconds for dnotify signal to be delivered
    let mut enable_checks = 0;
    while !(crate::common::is_hooked(&libc, "getlogin")) {
        assert!(enable_checks < 3);
        enable_checks += 1;
        std::thread::sleep(std::time::Duration::from_millis(35));
    }
    let generic_hook_addr: usize = unsafe { libc::dlsym(libc::RTLD_DEFAULT, "getlogin\0".as_ptr() as *const libc::c_char) } as usize;
    assert!(generic_hook_addr != 0);
    let hooked_getlogin: unsafe extern "C" fn() -> isize = unsafe { std::mem::transmute(generic_hook_addr) };
    let getlogin_result_hooked = unsafe { hooked_getlogin() } as *mut libc::c_char;
    // Clean up
    let sql_delete = r#"BEGIN;
                        DELETE FROM Hook WHERE symbol = "getlogin" AND library = (SELECT value FROM Setting WHERE param="SystemLibraryPath") || "libc.so.6";
                        COMMIT;"#;
    crate::common::load_sql(sql_delete);
    assert!(!(getlogin_result_hooked.is_null()));
    assert!(unsafe { libc::strncmp(getlogin_result_unhooked, getlogin_result_hooked, libc::strlen(getlogin_result_unhooked)) } == 0);
});

/*
whitebeam_test!("linux", interposition_16_generic_hook_six_args {
    // TODO: Load SQL hook with whitebeam command
    println!("Hello generic hook six arg test for Linux!");
});
*/

// TODO: Test for restarting wait() with SA_RESTART