build_action! { VerifyCanExecute (par_prog, src_prog, hook, arg_position, args, _act_args, do_return, return_value) {
        // TODO: Depending on LogSeverity, log all use of this action
        // TODO: Use OsString?
        let library: &str = &hook.library;
        let library_basename: &str = library.rsplit('/').next().unwrap_or(library);
        let symbol: &str = &hook.symbol;
        let any = String::from("ANY");
        let class = String::from("Filesystem/Path/Executable");
        let all_allowed_executables: Vec<String> = {
            let whitelist_cache_lock = crate::common::db::WL_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
            whitelist_cache_lock.iter().filter(|whitelist| (whitelist.class == class) && ((whitelist.parent == par_prog) || (whitelist.parent == any)) && ((whitelist.path == src_prog) || (whitelist.path == any))).map(|whitelist| whitelist.value.clone()).collect()
        };
        // Permit ANY
        if all_allowed_executables.iter().any(|executable| executable == &any) {
            return (hook, args, do_return, return_value);
        }
        let argument_index = arg_position.expect("WhiteBeam: Lost track of environment") as usize;
        let argument: crate::common::db::ArgumentRow = args[argument_index].clone();
        let target_executable: String = match (library_basename, symbol) {
            ("libc.so.6", "fexecve") => {
                let canonical_path = platform::canonicalize_fd(argument.real as i32).expect("WhiteBeam: Lost track of environment");
                canonical_path.into_os_string().into_string().expect("WhiteBeam: Unexpected null reference")
            },
            _ => unsafe { String::from(std::ffi::CStr::from_ptr(argument.real as *const libc::c_char).to_str().expect("WhiteBeam: Unexpected null reference")) }
        };
        // Ensure the target path is either a regular file or a symlink
        let mut stat_struct: libc::stat = unsafe { std::mem::zeroed() };
        let target_executable_null = format!("{}\0", target_executable);
        let path_stat = match unsafe { libc::stat(target_executable_null.as_ptr() as *const libc::c_char, &mut stat_struct) } {
          0 => Ok(stat_struct),
          _ => Err(()),
        };
        match path_stat {
            Ok(valid_path) => {
                if !(((valid_path.st_mode & libc::S_IFMT) == libc::S_IFREG) ||
                     ((valid_path.st_mode & libc::S_IFMT) == libc::S_IFLNK)) {
                    do_return = true;
                    return_value = -1;
                    platform::set_errno(libc::ENOENT);
                    return (hook, args, do_return, return_value);
                }
            }
            Err(_) => {
                do_return = true;
                return_value = -1;
                platform::set_errno(libc::ENOENT);
                return (hook, args, do_return, return_value);
            }
        }
        // Permit whitelisted executables
        if all_allowed_executables.iter().any(|executable| executable == &target_executable) {
            return (hook, args, do_return, return_value);
        }
        // Permit execution if not running in prevention mode
        if !(crate::common::db::get_prevention()) {
            event::send_log_event(libc::LOG_NOTICE, format!("Detection: {} -> {} executed {} (VerifyCanExecute)", &par_prog, &src_prog, &target_executable));
            return (hook, args, do_return, return_value);
        }
        // Permit authorized execution
        if crate::common::db::get_valid_auth_env() {
            return (hook, args, do_return, return_value);
        }
        // Deny by default
        event::send_log_event(libc::LOG_WARNING, format!("Prevention: Blocked {} -> {} from executing {} (VerifyCanExecute)", &par_prog, &src_prog, &target_executable));
        // TODO: Configurable verbosity here
        //eprintln!("WhiteBeam: {}: Permission denied", &target_executable);
        if (symbol.contains("exec") || symbol.contains("posix_spawn")) && (library_basename == "libc.so.6") {
            do_return = true;
            return_value = -1;
            platform::set_errno(libc::EPERM);
            return (hook, args, do_return, return_value);
        }
        do_return = true;
        return_value = -1;
}}
