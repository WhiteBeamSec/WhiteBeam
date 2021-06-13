#[macro_use]
build_action! { CanonicalizePath (_src_prog, hook, arg_id, args, do_return, return_value) {
        // TODO: Don't fatal if the Path cannot be canonicalized (return -1/0). NULL handling for dlopen?
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
        let file_index = args.iter().position(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment");
        let file_argument: crate::common::db::ArgumentRow = args[file_index].clone();
        let file_value = file_argument.real as *const libc::c_char;
        let new_file_value: std::ffi::OsString = match (library, symbol) {
            ("/lib/x86_64-linux-gnu/libdl.so.2", "dlopen") |
            ("/lib/x86_64-linux-gnu/libdl.so.2", "dlmopen") => {
                // TODO: Remove dependency on procfs here
                let fd: libc::c_int = unsafe { libc::open(file_value, libc::O_PATH) };
                if fd == -1 {
                    do_return = true;
                    return_value = 0;
                    return (hook, args, do_return, return_value);
                }
                let canonical_path = platform::canonicalize_fd(fd as i32).expect("WhiteBeam: Lost track of environment");
                canonical_path.into_os_string()
            },
            _ => {
                let file_osstring = unsafe { crate::common::convert::c_char_to_osstring(file_value) };
                match platform::search_path(&file_osstring) {
                    Some(abspath) => abspath.as_os_str().to_owned(),
                    None => {
                        unsafe { libc::exit(127) };
                    }
                }
            }
        };
        let new_file_value_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(&new_file_value).expect("WhiteBeam: Unexpected null reference"));
        args[file_index].datatype = String::from("String");
        args[file_index].real = Box::leak(new_file_value_cstring).as_ptr() as usize;
}}
