#[macro_use]
build_action! { CanonicalizePath (_src_prog, hook, arg_id, args, do_return, return_value) {
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
        let file_index = args.iter().position(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment");
        let file_argument: crate::common::db::ArgumentRow = args[file_index].clone();
        let new_file_value: std::ffi::OsString = match (library, symbol) {
            ("/lib/x86_64-linux-gnu/libc.so.6", "fexecve") => {
                let file_value = file_argument.real as i32;
                // TODO: Better validation here
                if !(0 <= file_value && file_value <= 1024) {
                    unsafe { libc::exit(127) };
                };
                // TODO: Remove dependency on procfs for libc fexecve
                match std::fs::read_link(format!("/proc/self/fd/{}", file_value)) {
                    Ok(abspath) => abspath.as_os_str().to_owned(),
                    Err(_e) => {
                        unsafe { libc::exit(127) };
                    }
                }
            },
            _ => {
                let file_value = file_argument.real as *const libc::c_char;
                let file_osstring = unsafe { crate::common::convert::c_char_to_osstring(file_value) };
                match crate::platforms::linux::search_path(&file_osstring) {
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
