#[macro_use]
build_action! { VerifyCanExecute (src_prog, hook, arg_id, args, do_return, return_value) {
        // TODO: Depending on LogVerbosity, log all use of this action
        // TODO: Use OsString?
        // Permit execution if not running in prevention mode
        if !(crate::common::db::get_prevention()) {
            return (hook, args, do_return, return_value);
        }
        // Permit authorized execution
        if crate::common::db::get_valid_auth_env() {
            return (hook, args, do_return, return_value);
        }
        let any = String::from("ANY");
        let class = String::from("Filesystem/Path/Executable");
        let all_allowed_executables: Vec<String> = {
            let whitelist_cache_lock = crate::common::db::WL_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
            whitelist_cache_lock.iter().filter(|whitelist| (whitelist.class == class) && ((whitelist.path == src_prog) || (whitelist.path == any))).map(|whitelist| whitelist.value.clone()).collect()
        };
        // Permit ANY
        if all_allowed_executables.iter().any(|executable| executable == &any) {
            return (hook, args, do_return, return_value);
        }
        let target_executable: String = {
            let argument: crate::common::db::ArgumentRow = args.iter().find(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment").clone();
            let canonical_path = platform::canonicalize_fd(argument.real as i32).expect("WhiteBeam: Lost track of environment");
            canonical_path.into_os_string().into_string().expect("WhiteBeam: Unexpected null reference")
        };
        // Permit whitelisted executables
        if all_allowed_executables.iter().any(|executable| executable == &target_executable) {
            return (hook, args, do_return, return_value);
        }
        // Deny by default
        if (&hook.symbol).contains("exec") && (&hook.library).contains("libc.so") {
            // Terminate the child process
            eprintln!("WhiteBeam: {}: Permission denied", &target_executable);
            unsafe { libc::exit(126) };
        }
        do_return = true;
        return_value = -1;
}}
