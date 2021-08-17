build_action! { SplitFilePath (_src_prog, hook, arg_id, args, do_return, return_value) {
        let path_index = args.iter().position(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment");
        let path_argument: crate::common::db::ArgumentRow = args[path_index].clone();
        let path_value = path_argument.real as *const libc::c_char;
        let path_osstring = unsafe { crate::common::convert::c_char_to_osstring(path_value) };
        let path_pathbuf: std::path::PathBuf = std::path::PathBuf::from(path_osstring);
        let path_abspathbuf: std::path::PathBuf = match path_pathbuf.is_absolute() {
            true => path_pathbuf,
            false => std::env::current_dir().expect("WhiteBeam: Lost track of environment").join(path_pathbuf)
        };
        let path_normal: std::path::PathBuf = crate::common::convert::normalize_path(&path_abspathbuf);
        // TODO: Error handling
        let basename: &std::ffi::OsStr = (&path_normal).file_name().unwrap_or(&std::ffi::OsStr::new("."));
        let basename_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(basename).expect("WhiteBeam: Unexpected null reference"));
        // TODO: Provide top level directory function in platform
        let dirfd: std::path::PathBuf = match (&path_normal).parent() {
            Some(f) => {
                if f == std::path::Path::new("") {
                    std::env::current_dir().expect("WhiteBeam: Lost track of environment")
                } else {
                    f.to_owned()
                }
            },
            None => std::path::PathBuf::from("/")
        };
        let dirfd_cstring: std::ffi::CString = crate::common::convert::osstr_to_cstring((&dirfd).as_os_str()).expect("WhiteBeam: Unexpected null reference");
        let fd: libc::c_int = unsafe { libc::open(dirfd_cstring.as_ptr(), libc::O_PATH) };
        if fd >= 0 {
            args[path_index].datatype = String::from("IntegerSigned");
            args[path_index].real = fd as usize;
            let new_arg = crate::common::db::ArgumentRow {
                hook: hook.id,
                parent: None,
                id: -1,
                position: path_index as i64,
                real: Box::leak(basename_cstring).as_ptr() as usize,
                datatype: String::from("String"),
                pointer: true,
                signed: false,
                variadic: false,
                array: false
            };
            args.insert(path_index+1, new_arg);
            return (hook, args, do_return, return_value);
        }
        do_return = true;
        return_value = -1;
}}
