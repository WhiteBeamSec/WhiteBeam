#[macro_use]
build_action! { VerifyCanExecute (src_prog, hook, arg_id, args, do_return, return_value) {
        // Permit execution if not running in prevention mode
        if !(crate::common::db::get_prevention()) {
            return (hook, args, do_return, return_value);
        }
        // Permit authorized execution
        if crate::common::db::get_valid_auth_env() {
            return (hook, args, do_return, return_value);
        }
        // Permit whitelisted executables
        let any = String::from("ANY");
        let class = String::from("Filesystem/Path/Executable");
        let all_allowed_executables: Vec<String> = {
            let whitelist_cache_lock = crate::common::db::WL_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
            whitelist_cache_lock.iter().filter(|whitelist| (whitelist.class == class) && ((whitelist.path == src_prog) || (whitelist.path == any))).map(|whitelist| whitelist.value.clone()).collect()
        };
        let target_executable: String = {
            let argument: crate::common::db::ArgumentRow = args.iter().find(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment").clone();
            String::from(unsafe { std::ffi::CStr::from_ptr(argument.real as *const i8) }.to_str().expect("WhiteBeam: Unexpected null reference"))
        };
        for executable in all_allowed_executables {
            // TODO: Consider removing references
            if (&target_executable == &executable) || (&any == &executable) {
                return (hook, args, do_return, return_value);
            }
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
