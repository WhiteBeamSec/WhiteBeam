#[macro_use]
build_action! { OpenFileDescriptor (_src_prog, hook, arg_id, args, do_return, return_value) {
        // TODO: No O_CLOEXEC leads to inherited fd's in children
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
        let file_index = args.iter().position(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment");
        let file_argument: crate::common::db::ArgumentRow = args[file_index].clone();
        let file_value = file_argument.real as *const libc::c_char;
        let flags: i32 = match (library, symbol) {
            // Execution: handled by default case
            // Filesystem
            ("/lib/x86_64-linux-gnu/libc.so.6", "fopen") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "fopen64") => {
                let mode_osstring: std::ffi::OsString = unsafe { crate::common::convert::c_char_to_osstring(args[file_index+1].clone().real as *const libc::c_char) };
                let mode_string = mode_osstring.into_string().expect("WhiteBeam: Unexpected null reference");
                // Ignore ",ccs=?"
                let mode_no_ccs = mode_string.splitn(2, ",").next().expect("WhiteBeam: Unexpected null reference");
                let mut glibc_extensions = 0;
                if mode_no_ccs.contains("e") { glibc_extensions |= libc::O_CLOEXEC };
                if mode_no_ccs.contains("x") { glibc_extensions |= libc::O_EXCL };
                let mode_clean = mode_no_ccs.replace(&['b', 'c', 'e', 'm', 'x'][..], "");
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
            ("/lib/x86_64-linux-gnu/libc.so.6", "truncate") => {
                let length: i64 = args[file_index+1].clone().real as i64;
                match length {
                    0 => libc::O_WRONLY | libc::O_TRUNC,
                    _ => libc::O_WRONLY
                }
            },
            _ => libc::O_PATH
        };
        let fd: libc::c_int = unsafe { libc::open(file_value, flags) };
        if fd >= 0 {
            args[file_index].datatype = String::from("IntegerSigned");
            args[file_index].real = fd as usize;
            return (hook, args, do_return, return_value);
        }
        do_return = true;
        match (library, symbol) {
            ("/lib/x86_64-linux-gnu/libc.so.6", "fopen") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "fopen64") => {
                return_value = 0;
            }
            _ => {
                return_value = -1;
            }
        };
}}
