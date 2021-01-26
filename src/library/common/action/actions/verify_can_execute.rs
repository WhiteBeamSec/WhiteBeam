#[macro_use]
build_action! { verify_can_execute (src_prog, hooked_fn, arg_id, args, do_return, return_value) {
        // Permit execution if not running in protected mode
        if !(crate::common::db::get_protected()) {
            return (hooked_fn, args, do_return, return_value);
        }
        // Permit authorized execution
        if crate::common::db::get_valid_auth_env() {
            return (hooked_fn, args, do_return, return_value);
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
                return (hooked_fn, args, do_return, return_value);
            }
        }
        // Deny by default
        do_return = true;
        return_value = -1;
}}
