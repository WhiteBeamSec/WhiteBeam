fn get_restricted() -> Vec<std::ffi::OsString> {
    vec!(
        std::ffi::OsString::from("LD_PROFILE"),
        std::ffi::OsString::from("LD_PROFILE_OUTPUT"),
        std::ffi::OsString::from("LD_DEBUG_OUTPUT")
    )
}

build_action! { FilterEnvironment (_src_prog, hook, arg_id, args, do_return, return_value) {
        // Enforce LD_AUDIT, LD_BIND_NOT, WB_PROG
        // TODO: Avoid leaking memory (NB: this action is often called before execve on Linux)
        let library: &str = &hook.library;
        let library_basename: &str = library.rsplit('/').next().unwrap_or(library);
        let symbol: &str = &hook.symbol;
        let envp_index: usize = {
            // Non-positional functions
            match (library_basename, symbol) {
                ("libc.so.6", "execl") |
                ("libc.so.6", "execlp") |
                ("libc.so.6", "execv") |
                ("libc.so.6", "execvp") => {
                    args.iter().position(|arg| arg.id == -1).expect("WhiteBeam: Lost track of environment")
                }
                _ => {
                    args.iter().position(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment")
                }
            }
        };
        let envp_argument: crate::common::db::ArgumentRow = args[envp_index].clone();
        let envp = envp_argument.real as *const *const libc::c_char;
        let orig_env_vec = unsafe {
            let mut env: Vec<(&std::ffi::OsStr, &std::ffi::OsStr)> = Vec::new();
            if !(envp.is_null()) {
                let mut envp_iter = envp;
                while !(*envp_iter).is_null() {
                    let input = std::ffi::CStr::from_ptr(*envp_iter).to_bytes();
                    if !input.is_empty() {
                    		match input[1..].iter().position(|&x| x == b'=').map(|p| p + 1) {
                                Some(p) => {
                                    env.push((crate::common::convert::u8_slice_as_os_str(&input[..p]),
                                              crate::common::convert::u8_slice_as_os_str(&input[p + 1..])));
                                },
                                None => {
                                    // TODO: Log
                                }
                            };
                    }
                    envp_iter = envp_iter.add(1);
                }
            }
            env
        };
        let mut update_ld_audit: bool = false;
        let mut update_ld_bind_not: bool = false;
        // TODO: Support more platforms here
        let rtld_audit_lib_path = crate::platforms::linux::get_rtld_audit_lib_path();
        let new_ld_audit_var: std::ffi::OsString = match orig_env_vec.iter().find(|var| var.0 == "LD_AUDIT") {
            Some(val) => {
                if crate::common::convert::osstr_split_at_byte(val.1, b':').0 == rtld_audit_lib_path {
                    std::ffi::OsString::new()
                } else {
                    update_ld_audit = true;
                    let mut new_ld_audit_osstring = std::ffi::OsString::from("LD_AUDIT=");
                    new_ld_audit_osstring.push(rtld_audit_lib_path.as_os_str());
                    new_ld_audit_osstring.push(std::ffi::OsStr::new(":"));
                    new_ld_audit_osstring.push(val.1);
                    new_ld_audit_osstring
                }
            }
            None => {
                update_ld_audit = true;
                let mut new_ld_audit_osstring = std::ffi::OsString::from("LD_AUDIT=");
                new_ld_audit_osstring.push(rtld_audit_lib_path.as_os_str());
                new_ld_audit_osstring
            }
        };
        let new_ld_bind_not_var: std::ffi::OsString = match orig_env_vec.iter().find(|var| var.0 == "LD_BIND_NOT") {
            Some(val) => {
                if val.1 != "1" {
                    update_ld_bind_not = true;
                    std::ffi::OsString::from("LD_BIND_NOT=1")
                } else {
                    std::ffi::OsString::new()
                }
            }
            None => {
                update_ld_bind_not = true;
                std::ffi::OsString::from("LD_BIND_NOT=1")
            }
        };
        let mut env_vec: Vec<*const libc::c_char> = Vec::new();
        let program_path: std::ffi::OsString = match (library_basename, symbol) {
            ("libc.so.6", "fexecve") => platform::canonicalize_fd(args[0].real as i32).expect("WhiteBeam: Lost track of environment").into_os_string(),
            _ => unsafe { crate::common::convert::c_char_to_osstring(args[0].real as *const libc::c_char) }
        };
        if update_ld_audit {
            // TODO: Log null reference, process errors
            let new_ld_audit_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(&new_ld_audit_var).expect("WhiteBeam: Unexpected null reference"));
            // TODO: Check whitelist for path
            env_vec.push(Box::leak(new_ld_audit_cstring).as_ptr());
        }
        if update_ld_bind_not {
            // TODO: Log null reference, process errors
            let new_ld_bind_not_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(&new_ld_bind_not_var).expect("WhiteBeam: Unexpected null reference"));
            env_vec.push(Box::leak(new_ld_bind_not_cstring).as_ptr());
        }
        let mut program_path_env: std::ffi::OsString = std::ffi::OsString::from("WB_PROG=");
        program_path_env.push(&program_path);
        let program_path_env_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(&program_path_env).expect("WhiteBeam: Unexpected null reference"));
        env_vec.push(Box::leak(program_path_env_cstring).as_ptr());
        unsafe {
        if !(envp.is_null()) {
            let mut envp_iter = envp;
            while !(*envp_iter).is_null() {
                if let Some(key_value) = crate::common::convert::parse_env_single(std::ffi::CStr::from_ptr(*envp_iter).to_bytes()) {
                    if get_restricted().contains(&key_value.0) {
                         panic!("WhiteBeam: LD output variables restricted");
                    }
                    if  (!(update_ld_audit) && (key_value.0 == std::ffi::OsString::from("LD_AUDIT")))
                     || (!(update_ld_bind_not) && (key_value.0 == std::ffi::OsString::from("LD_BIND_NOT")))
                     || ((key_value.0 != std::ffi::OsString::from("LD_AUDIT")) && (key_value.0 != std::ffi::OsString::from("LD_BIND_NOT")) && (key_value.0 != std::ffi::OsString::from("WB_PROG"))) {
                        env_vec.push(*envp_iter);
                    }
                }
                envp_iter = envp_iter.offset(1);
            }
        }
        }
        env_vec.push(std::ptr::null());
        args[envp_index].real = Box::leak(env_vec.into_boxed_slice()).as_ptr() as usize;
}}
