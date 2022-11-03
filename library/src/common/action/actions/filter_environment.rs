use std::os::unix::ffi::OsStrExt;

fn osstr_split_at_colon_or_semicolon(osstr_input: &std::ffi::OsStr) -> (&std::ffi::OsStr, &std::ffi::OsStr) {
    for (i, b) in osstr_input.as_bytes().iter().enumerate() {
        if b == &b':' || b == &b';' {
            return (std::ffi::OsStr::from_bytes(&osstr_input.as_bytes()[..i]),
                std::ffi::OsStr::from_bytes(&osstr_input.as_bytes()[i + 1..]));
        }
    }
    (&*osstr_input, std::ffi::OsStr::from_bytes(&osstr_input.as_bytes()[osstr_input.len()..osstr_input.len()]))
}

fn collect_ld_library_paths(osstr_input: &std::ffi::OsStr) -> Vec<std::ffi::OsString> {
    let mut vec: Vec<std::ffi::OsString> = Vec::new();
    let mut osstr_in = osstr_input;
    loop {
        let (osstr_left, osstr_right) = osstr_split_at_colon_or_semicolon(osstr_in);
        if osstr_left.len() == 0 {
            match std::env::current_dir() {
                Ok(cwd) => {
                    let mut cwd_osstring = crate::common::convert::normalize_path(&std::path::PathBuf::from(cwd)).into_os_string();
                    cwd_osstring.push("/");
                    if !(vec.contains(&cwd_osstring)) {
                        vec.push(cwd_osstring)
                    }
                },
                Err(_val) => () // TODO: Log errors
            }
        } else {
            let mut osstring_left = crate::common::convert::normalize_path(&std::path::PathBuf::from(osstr_left)).into_os_string();
            osstring_left.push("/");
            if !(vec.contains(&osstring_left)) {
                vec.push(osstring_left)
            }
        }
        if osstr_right.len() == 0 {
            break;
        }
        osstr_in = osstr_right;
    }
    let last_character = osstr_input.as_bytes().last();
    if last_character == Some(&b':') || last_character == Some(&b';') {
        match std::env::current_dir() {
            Ok(cwd) => {
                let mut cwd_osstring = crate::common::convert::normalize_path(&std::path::PathBuf::from(cwd)).into_os_string();
                cwd_osstring.push("/");
                if !(vec.contains(&cwd_osstring)) {
                    vec.push(cwd_osstring)
                }
            },
            Err(_val) => () // TODO: Log errors
        }
    }
    vec
}

build_action! { FilterEnvironment (par_prog, src_prog, hook, arg_position, args, _act_args, do_return, return_value) {
        // TODO: Refactor for efficiency
        // TODO: Eliminate code repetition
        // Enforce LD_AUDIT, LD_LIBRARY_PATH, LD_PROFILE, LD_PROFILE_OUTPUT, LD_DEBUG_OUTPUT, LD_BIND_NOT, WB_PARENT, and WB_PROG
        // TODO: Avoid leaking memory (NB: this action is often called before execve on Linux)
        let library: &str = &hook.library;
        let library_basename: &str = library.rsplit('/').next().unwrap_or(library);
        let symbol: &str = &hook.symbol;
        let envp_index = arg_position.expect("WhiteBeam: Lost track of environment") as usize;
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
        let mut update_ld_library_path: bool = false;
        let mut update_ld_profile: bool = false;
        let mut update_ld_profile_output: bool = false;
        let mut update_ld_debug_output: bool = false;
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
        let any = String::from("ANY");
        let new_ld_library_path_var: std::ffi::OsString = match orig_env_vec.iter().find(|var| var.0 == "LD_LIBRARY_PATH") {
            Some(val) => {
                if val.1.len() == 0 {
                    std::ffi::OsString::new()
                } else {
                    update_ld_library_path = true;
                    // Collect whitelisted Filesystem/Path/Library entries
                    let library_class = String::from("Filesystem/Path/Library");
                    let all_allowed_libraries: Vec<String> = {
                        let whitelist_cache_lock = crate::common::db::WL_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
                        whitelist_cache_lock.iter().filter(|whitelist| (whitelist.class == library_class) && ((whitelist.parent == par_prog) || (whitelist.parent == any)) && ((whitelist.path == src_prog) || (whitelist.path == any))).map(|whitelist| whitelist.value.clone()).collect()
                    };
                    let lib_paths: Vec<std::ffi::OsString> = collect_ld_library_paths(val.1);
                    let mut permitted_lib_paths: Vec<std::ffi::OsString> = vec![];
                    // Permit whitelisted library paths
                    for lib_path in lib_paths.iter() {
                        if all_allowed_libraries.iter().any(|allowed_library| glob::Pattern::new(allowed_library).expect("WhiteBeam: Invalid glob pattern").matches_path(&std::path::PathBuf::from(&lib_path))) {
                            permitted_lib_paths.push(lib_path.clone());
                        } else if !(crate::common::db::get_prevention()) {
                            let lib_path_string = lib_path.clone().into_string().expect("WhiteBeam: Unexpected null reference");
                            event::send_log_event(libc::LOG_NOTICE, format!("Detection: {} -> {} inserted library path {} (FilterEnvironment)", &par_prog, &src_prog, lib_path_string));
                            permitted_lib_paths.push(lib_path.clone());
                        } else {
                            let lib_path_string = lib_path.clone().into_string().expect("WhiteBeam: Unexpected null reference");
                            event::send_log_event(libc::LOG_WARNING, format!("Prevention: Blocked {} -> {} from inserting library path {} (FilterEnvironment)", &par_prog, &src_prog, lib_path_string));
                        }
                    }
                    // Concatenate permitted paths
                    let mut new_ld_library_path_osstring = std::ffi::OsString::from("LD_LIBRARY_PATH=");
                    let new_ld_library_path_value: std::ffi::OsString = permitted_lib_paths.join(std::ffi::OsString::from(":").as_ref());
                    new_ld_library_path_osstring.push(new_ld_library_path_value);
                    new_ld_library_path_osstring
                }
            }
            None => { std::ffi::OsString::new() }
        };
        // Get list of whitelisted Filesystem/Path/Writable entries if LD_PROFILE, LD_PROFILE_OUTPUT, or LD_DEBUG_OUTPUT are set
        let writable_variables: Vec<&std::ffi::OsStr> = vec![std::ffi::OsStr::new("LD_PROFILE"), std::ffi::OsStr::new("LD_PROFILE_OUTPUT"), std::ffi::OsStr::new("LD_DEBUG_OUTPUT")];
        let ld_output_set: bool = orig_env_vec.iter().any(|var| writable_variables.contains(&(var.0)));
        let mut all_allowed_writable_paths: Vec<String> = vec![];
        if ld_output_set {
            let filesystem_class = String::from("Filesystem/Directory/Writable");
            all_allowed_writable_paths = {
                let whitelist_cache_lock = crate::common::db::WL_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
                whitelist_cache_lock.iter().filter(|whitelist| (whitelist.class == filesystem_class) && ((whitelist.parent == par_prog) || (whitelist.parent == any)) && ((whitelist.path == src_prog) || (whitelist.path == any))).map(|whitelist| whitelist.value.clone()).collect()
            };
        }
        let new_ld_profile_output_var: std::ffi::OsString = std::ffi::OsString::from("LD_PROFILE_OUTPUT=");
        let new_ld_profile_var: std::ffi::OsString = match orig_env_vec.iter().find(|var| var.0 == "LD_PROFILE") {
            Some(val) => {
                // TODO: /tmp/../ is normalized to // instead of /
                let mut ld_profile_output_value: std::ffi::OsString = match orig_env_vec.iter().find(|out_var| (out_var.0 == "LD_PROFILE_OUTPUT") && (out_var.1.len() > 0)) {
                    Some(out_val) => {
                        let mut out_buffer = out_val.1.to_os_string();
                        out_buffer.push("/");
                        out_buffer
                    }
                    None => {
                        let at_secure: bool = unsafe { libc::getauxval(libc::AT_SECURE) } == 1;
                        if at_secure {
                            std::ffi::OsString::from("/var/profile/")
                        } else {
                            std::ffi::OsString::from("/var/tmp/")
                        }
                    }
                };
                let ld_profile_value: std::ffi::OsString = format!("{}.profile", val.1.to_str().expect("WhiteBeam: Unexpected null reference")).into();
                ld_profile_output_value.push(ld_profile_value);
                let normalized_writable_path: std::path::PathBuf = crate::common::convert::normalize_path(&std::path::PathBuf::from(ld_profile_output_value));
                let root_path = std::path::PathBuf::from("/");
                let writable_path_dir: std::path::PathBuf = match normalized_writable_path.parent() {
                    Some(parent) => {
                        let mut parent_buffer = parent.to_owned().into_os_string();
                        parent_buffer.push("/");
                        std::path::PathBuf::from(parent_buffer)
                    }
                    None => { root_path }
                };
                // Permit whitelisted writable paths
                if all_allowed_writable_paths.iter().any(|allowed_writable_path| glob::Pattern::new(allowed_writable_path).expect("WhiteBeam: Invalid glob pattern").matches_path(&writable_path_dir)) {
                    std::ffi::OsString::new()
                } else if !(crate::common::db::get_prevention()) {
                    event::send_log_event(libc::LOG_NOTICE, format!("Detection: {} -> {} wrote to {} (FilterEnvironment)", &par_prog, &src_prog, writable_path_dir.display()));
                    std::ffi::OsString::new()
                } else {
                    update_ld_profile = true;
                    update_ld_profile_output = true;
                    event::send_log_event(libc::LOG_WARNING, format!("Prevention: Blocked {} -> {} from writing to {} (FilterEnvironment)", &par_prog, &src_prog, writable_path_dir.display()));
                    std::ffi::OsString::from("LD_PROFILE=")
                }
            }
            None => { std::ffi::OsString::new() }
        };
        if orig_env_vec.iter().any(|var| var.0 == "LD_DEBUG") {
            if let Some(val) = orig_env_vec.iter().find(|var| var.0 == "LD_DEBUG_OUTPUT") {
                // TODO: LD_DEBUG_OUTPUT of /anything results in writable_path_dir being set to //
                let mut normalized_writable_path: std::path::PathBuf = crate::common::convert::normalize_path(&std::path::PathBuf::from(&(val.1)));
                if !(normalized_writable_path.is_absolute()) {
                    match std::env::current_dir() {
                        Ok(cwd) => {
                            let mut cwd_osstring = crate::common::convert::normalize_path(&std::path::PathBuf::from(cwd)).into_os_string();
                            cwd_osstring.push("/");
                            normalized_writable_path = std::path::PathBuf::from(&cwd_osstring).join(normalized_writable_path);
                        },
                        Err(_val) => () // TODO: Log errors
                    }
                }
                let writable_path_dir: std::path::PathBuf = if !(val.1.is_empty()) {
                    let root_path = std::path::PathBuf::from("/");
                    match normalized_writable_path.parent() {
                        Some(parent) => {
                            let mut parent_buffer = parent.to_owned().into_os_string();
                            parent_buffer.push("/");
                            std::path::PathBuf::from(parent_buffer)
                        }
                        None => { root_path }
                    }
                } else {
                    normalized_writable_path
                };
                if !(all_allowed_writable_paths.iter().any(|allowed_writable_path| glob::Pattern::new(allowed_writable_path).expect("WhiteBeam: Invalid glob pattern").matches_path(&writable_path_dir))) {
                    // Not a whitelisted writable path
                    if !(crate::common::db::get_prevention()) {
                        event::send_log_event(libc::LOG_NOTICE, format!("Detection: {} -> {} wrote to {} (FilterEnvironment)", &par_prog, &src_prog, writable_path_dir.display()));
                    } else {
                        // Delete the LD_DEBUG_OUTPUT variable. An empty LD_DEBUG_OUTPUT writes to the cwd.
                        update_ld_debug_output = true;
                        event::send_log_event(libc::LOG_WARNING, format!("Prevention: Blocked {} -> {} from writing to {} (FilterEnvironment)", &par_prog, &src_prog, writable_path_dir.display()));
                    }
                }
            };
        }
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
            ("libc.so.6", "posix_spawn") |
            ("libc.so.6", "posix_spawnp") => {
                unsafe { crate::common::convert::c_char_to_osstring(args[1].real as *const libc::c_char) }
            }
            _ => unsafe { crate::common::convert::c_char_to_osstring(args[0].real as *const libc::c_char) }
        };
        if update_ld_audit {
            // TODO: Log null reference, process errors
            let new_ld_audit_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(&new_ld_audit_var).expect("WhiteBeam: Unexpected null reference"));
            // TODO: Check whitelist for path
            env_vec.push(Box::leak(new_ld_audit_cstring).as_ptr());
        }
        if update_ld_library_path {
            // TODO: Log null reference, process errors
            let new_ld_library_path_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(&new_ld_library_path_var).expect("WhiteBeam: Unexpected null reference"));
            env_vec.push(Box::leak(new_ld_library_path_cstring).as_ptr());
        }
        if update_ld_profile {
            // TODO: Log null reference, process errors
            let new_ld_profile_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(&new_ld_profile_var).expect("WhiteBeam: Unexpected null reference"));
            env_vec.push(Box::leak(new_ld_profile_cstring).as_ptr());
        }
        if update_ld_profile_output {
            // TODO: Log null reference, process errors
            let new_ld_profile_output_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(&new_ld_profile_output_var).expect("WhiteBeam: Unexpected null reference"));
            env_vec.push(Box::leak(new_ld_profile_output_cstring).as_ptr());
        }
        if update_ld_bind_not {
            // TODO: Log null reference, process errors
            let new_ld_bind_not_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(&new_ld_bind_not_var).expect("WhiteBeam: Unexpected null reference"));
            env_vec.push(Box::leak(new_ld_bind_not_cstring).as_ptr());
        }
        let mut parent_path_env: std::ffi::OsString = std::ffi::OsString::from("WB_PARENT=");
        parent_path_env.push(&src_prog);
        let parent_path_env_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(&parent_path_env).expect("WhiteBeam: Unexpected null reference"));
        env_vec.push(Box::leak(parent_path_env_cstring).as_ptr());
        let mut program_path_env: std::ffi::OsString = std::ffi::OsString::from("WB_PROG=");
        program_path_env.push(&program_path);
        let program_path_env_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(&program_path_env).expect("WhiteBeam: Unexpected null reference"));
        env_vec.push(Box::leak(program_path_env_cstring).as_ptr());
        unsafe {
        if !(envp.is_null()) {
            let mut envp_iter = envp;
            while !(*envp_iter).is_null() {
                if let Some(key_value) = crate::common::convert::parse_env_single(std::ffi::CStr::from_ptr(*envp_iter).to_bytes()) {
                    if  (!(update_ld_audit) && (key_value.0 == std::ffi::OsString::from("LD_AUDIT")))
                     || (!(update_ld_library_path) && (key_value.0 == std::ffi::OsString::from("LD_LIBRARY_PATH")))
                     || (!(update_ld_profile) && (key_value.0 == std::ffi::OsString::from("LD_PROFILE")))
                     || (!(update_ld_profile_output) && (key_value.0 == std::ffi::OsString::from("LD_PROFILE_OUTPUT")))
                     || (!(update_ld_debug_output) && (key_value.0 == std::ffi::OsString::from("LD_DEBUG_OUTPUT")))
                     || (!(update_ld_bind_not) && (key_value.0 == std::ffi::OsString::from("LD_BIND_NOT")))
                     || ((key_value.0 != std::ffi::OsString::from("LD_AUDIT")) &&
                         (key_value.0 != std::ffi::OsString::from("LD_LIBRARY_PATH")) &&
                         (key_value.0 != std::ffi::OsString::from("LD_PROFILE")) &&
                         (key_value.0 != std::ffi::OsString::from("LD_PROFILE_OUTPUT")) &&
                         (key_value.0 != std::ffi::OsString::from("LD_DEBUG_OUTPUT")) &&
                         (key_value.0 != std::ffi::OsString::from("LD_BIND_NOT")) &&
                         (key_value.0 != std::ffi::OsString::from("WB_PARENT")) &&
                         (key_value.0 != std::ffi::OsString::from("WB_PROG"))) {
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
