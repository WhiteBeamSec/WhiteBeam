#[macro_use]
build_action! { ConsumeVariadic (_src_prog, hook, arg_id, args, do_return, return_value) {
        let variadic_start = args.iter().position(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment");
        let variadic_start_id: i64 = args[variadic_start].id;
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
        let va_arg_iter: Vec<&crate::common::db::ArgumentRow> = args.iter().filter(|arg| arg.variadic && (arg.id == variadic_start_id)).collect();
        let va_arg_iter_len = va_arg_iter.len();
        match (library, symbol) {
            ("/lib/x86_64-linux-gnu/libc.so.6", "execl") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "execle") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "execlp") => {
                assert!(va_arg_iter_len > 0, "WhiteBeam: Insufficient arguments to ConsumeVariadic action");
                let mut argv_vec: Vec<*const libc::c_char> = Vec::new();
                for arg in va_arg_iter {
                    argv_vec.push(arg.real as *const libc::c_char);
                }
                args[variadic_start].real = (&argv_vec).as_ptr() as usize;
                args[variadic_start].datatype = String::from("StringArray");
                args[variadic_start].variadic = false;
                args[variadic_start].array = true;
                args.retain(|arg| !(arg.variadic && (arg.id == variadic_start_id)));
                // TODO: Update the position of the following arguments
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "open") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "open64") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "openat") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "openat64") => {
                assert!(va_arg_iter_len > 1, "WhiteBeam: Insufficient arguments to ConsumeVariadic action");
                args.truncate(va_arg_iter_len-2)
            },
            _ => { unimplemented!("WhiteBeam: The '{}' symbol (from {}) is not supported by the ConsumeVariadic action", symbol, library) }
        };
}}
