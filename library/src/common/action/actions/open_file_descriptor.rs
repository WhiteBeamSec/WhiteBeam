fn fail(library_basename: &str, symbol: &str) -> isize {
    match (library_basename, symbol) {
        ("libc.so.6", "fopen") |
        ("libc.so.6", "fopen64") => 0,
        _ => -1
    }
}

build_action! { OpenFileDescriptor (src_prog, hook, arg_id, args, _act_args, do_return, return_value) {
        // TODO: Refactor
        // TODO: No O_CLOEXEC leads to inherited fd's in children
        let library: &str = &hook.library;
        let library_basename: &str = library.rsplit('/').next().unwrap_or(library);
        let symbol: &str = &hook.symbol;
        let file_index = args.iter().position(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment");
        let file_argument: crate::common::db::ArgumentRow = args[file_index].clone();
        let file_value = file_argument.real as *const libc::c_char;
        let flags: i32 = match (library_basename, symbol) {
            // Filesystem
            ("libc.so.6", "fopen") |
            ("libc.so.6", "fopen64") => {
                let mode_osstring: std::ffi::OsString = unsafe { crate::common::convert::c_char_to_osstring(args[file_index+1].clone().real as *const libc::c_char) };
                let mode_string = mode_osstring.into_string().expect("WhiteBeam: Unexpected null reference");
                // Ignore ",ccs=?"
                let mode_no_ccs = mode_string.splitn(2, ",").next().expect("WhiteBeam: Unexpected null reference");
                let mut glibc_extensions = 0;
                if mode_no_ccs.contains("e") { glibc_extensions |= libc::O_CLOEXEC };
                if mode_no_ccs.contains("x") { glibc_extensions |= libc::O_EXCL };
                let mode_clean = mode_no_ccs.replace(&['b', 'c', 'e', 'm', 't', 'x'][..], "");
                // fopen() mode => open() flags
                let regular_flags: i32 = match mode_clean.as_ref() {
                    "r"  => libc::O_RDONLY,
                    "w"  => libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC,
                    "a"  => libc::O_WRONLY | libc::O_CREAT | libc::O_APPEND,
                    "r+" => libc::O_RDWR,
                    "w+" => libc::O_RDWR | libc::O_CREAT | libc::O_TRUNC,
                    "a+" => libc::O_RDWR | libc::O_CREAT | libc::O_APPEND,
                    _ => {
                        do_return = true;
                        return_value = 0;
                        unsafe { *platform::errno_location() = libc::EINVAL };
                        return (hook, args, do_return, return_value);
                    }
                };
                regular_flags | glibc_extensions
            },
            ("libc.so.6", "truncate") |
            ("libc.so.6", "truncate64") => {
                let length: i64 = args[file_index+1].clone().real as i64;
                match length {
                    0 => libc::O_WRONLY | libc::O_TRUNC,
                    _ => libc::O_WRONLY
                }
            },
            _ => libc::O_PATH
        };
        let file_osstring = unsafe { crate::common::convert::c_char_to_osstring(file_value) };
        let file_pathbuf: std::path::PathBuf = std::path::PathBuf::from(file_osstring);
        let file_normal: std::path::PathBuf = crate::common::convert::normalize_path(&file_pathbuf);
        // TODO: Error handling
        let basename: &std::ffi::OsStr = (&file_normal).file_name().unwrap_or(&std::ffi::OsStr::new("."));
        let basename_cstring: std::ffi::CString = crate::common::convert::osstr_to_cstring(basename).expect("WhiteBeam: Unexpected null reference");
        // TODO: Provide top level directory function in platform
        let dirfd_pathbuf: std::path::PathBuf = match (&file_normal).parent() {
            Some(f) => {
                if f == std::path::Path::new("") {
                    std::env::current_dir().expect("WhiteBeam: Lost track of environment")
                } else {
                    f.to_owned()
                }
            },
            None => std::path::PathBuf::from("/")
        };
        let dirfd_cstring: std::ffi::CString = crate::common::convert::osstr_to_cstring((&dirfd_pathbuf).as_os_str()).expect("WhiteBeam: Unexpected null reference");
        let dirfd: libc::c_int = unsafe { libc::open(dirfd_cstring.as_ptr(), libc::O_PATH) };
        // The operating system masks the default permissions with the umask to produce the final permissions.
        let default_permissions: libc::mode_t = 0o666;
        let need_permissions: bool = (flags & (libc::O_CREAT | libc::O_TMPFILE)) > 0;
        const TMPFILE_MINUS_DIRECTORY: libc::c_int = 0o20000000;
        let is_read_only: bool = !((flags & (libc::O_RDWR | libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | TMPFILE_MINUS_DIRECTORY | libc::O_APPEND | libc::O_TRUNC)) > 0);
        // Permit read-only
        if is_read_only {
            let fd: libc::c_int = unsafe { libc::open(file_value, flags) };
            if fd >= 0 {
                args[file_index].datatype = String::from("IntegerSigned");
                args[file_index].real = fd as usize;
                return (hook, args, do_return, return_value);
            }
            do_return = true;
            return_value = fail(library_basename, symbol);
            return (hook, args, do_return, return_value);
        }
        let any = String::from("ANY");
        let class = String::from("Filesystem/Directory/Writable");
        let all_allowed_directories: Vec<String> = {
            let whitelist_cache_lock = crate::common::db::WL_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
            whitelist_cache_lock.iter().filter(|whitelist| (whitelist.class == class) && ((whitelist.path == src_prog) || (whitelist.path == any))).map(|whitelist| whitelist.value.clone()).collect()
        };
        // Permit ANY
        if all_allowed_directories.iter().any(|directory| directory == &any) {
            let fd: libc::c_int = match need_permissions {
                true => unsafe { libc::openat(dirfd, basename_cstring.as_ptr(), flags, default_permissions) },
                false => unsafe { libc::openat(dirfd, basename_cstring.as_ptr(), flags) }
            };
            if fd >= 0 {
                args[file_index].datatype = String::from("IntegerSigned");
                args[file_index].real = fd as usize;
                return (hook, args, do_return, return_value);
            }
            do_return = true;
            return_value = fail(library_basename, symbol);
            return (hook, args, do_return, return_value);
        }
        // NB: Do not dereference paths here
        let canonical_path = match platform::canonicalize_fd(dirfd as i32) {
            Some(p) => p,
            None => {
                do_return = true;
                return_value = fail(library_basename, symbol);
                return (hook, args, do_return, return_value);
            }
        };
        // Minor performance hit by defining here instead of match statement
        let filename: String = String::from(basename.to_str().expect("WhiteBeam: Unexpected null reference"));
        let mut target_directory: String = canonical_path.into_os_string().into_string().expect("WhiteBeam: Unexpected null reference");
        target_directory.push('/');
        let full_path = format!("{}{}", target_directory, filename);
        // Special cases. We don't want to whitelist /dev (although pts and related subdirectories are fine).
        if (full_path == "/dev/tty") || (full_path == "/dev/null") {
            let fd: libc::c_int = match need_permissions {
                true => unsafe { libc::openat(dirfd, basename_cstring.as_ptr(), flags, default_permissions) },
                false => unsafe { libc::openat(dirfd, basename_cstring.as_ptr(), flags) }
            };
            if fd >= 0 {
                args[file_index].datatype = String::from("IntegerSigned");
                args[file_index].real = fd as usize;
                return (hook, args, do_return, return_value);
            }
            do_return = true;
            return_value = fail(library_basename, symbol);
            return (hook, args, do_return, return_value);
        }
        // Permit whitelisted directories
        if all_allowed_directories.iter().any(|directory| glob::Pattern::new(directory).expect("WhiteBeam: Invalid glob pattern").matches(&target_directory)) {
            let fd: libc::c_int = match need_permissions {
                true => unsafe { libc::openat(dirfd, basename_cstring.as_ptr(), flags, default_permissions) },
                false => unsafe { libc::openat(dirfd, basename_cstring.as_ptr(), flags) }
            };
            if fd >= 0 {
                args[file_index].datatype = String::from("IntegerSigned");
                args[file_index].real = fd as usize;
                return (hook, args, do_return, return_value);
            }
            do_return = true;
            return_value = fail(library_basename, symbol);
            return (hook, args, do_return, return_value);
        }
        // Permit authorized writes
        if (!(crate::common::db::get_prevention())) || crate::common::db::get_valid_auth_env() {
            if !(crate::common::db::get_prevention()) {
                event::send_log_event(syslog::Severity::LOG_NOTICE as i64, format!("Detection: {} wrote to {} (OpenFileDescriptor)", &src_prog, &target_directory));
            }
            let fd: libc::c_int = match need_permissions {
                true => unsafe { libc::open(file_value, flags, default_permissions) },
                false => unsafe { libc::open(file_value, flags) }
            };
            if fd >= 0 {
                args[file_index].datatype = String::from("IntegerSigned");
                args[file_index].real = fd as usize;
                return (hook, args, do_return, return_value);
            }
            do_return = true;
            return_value = fail(library_basename, symbol);
            return (hook, args, do_return, return_value);
        }
        // Deny by default
        event::send_log_event(syslog::Severity::LOG_WARNING as i64, format!("Prevention: Blocked {} from writing to {} (OpenFileDescriptor)", &src_prog, &target_directory));
        eprintln!("WhiteBeam: {}: Permission denied", &full_path);
        do_return = true;
        return_value = fail(library_basename, symbol);
}}
