#[macro_use]
build_action! { CanonicalizePath (_src_prog, hook, arg_id, args, do_return, return_value) {
        let file_index = args.iter().position(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment");
        let file_argument: crate::common::db::ArgumentRow = args[file_index].clone();
        let is_libc_fexecve: bool = ((&hook.symbol) == "fexecve") && (&hook.library).contains("libc.so");
        let new_file_value: std::ffi::OsString = {
            if is_libc_fexecve {
                let file_value = file_argument.real as i32;
                // TODO: Remove dependency on procfs for libc fexecve
                std::ffi::OsString::from(format!("/proc/self/fd/{}", file_value))
            } else {
                unsafe {
                let file_value = file_argument.real as *const *const libc::c_char;
                let file_osstr = crate::common::convert::c_char_to_osstring(*file_value);
                let absolute_path = match crate::platforms::linux::search_path(&file_osstr) {
                    Some(abspath) => abspath,
                    None => {
                        libc::exit(127);
                    }
                };
                absolute_path.as_os_str().to_owned()
                }
            }
        };
        let new_file_value_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(&new_file_value).expect("WhiteBeam: Unexpected null reference"));
        args[file_index].datatype = String::from("String");
        args[file_index].real = Box::leak(new_file_value_cstring).as_ptr() as usize;
}}
