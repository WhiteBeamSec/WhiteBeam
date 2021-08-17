build_action! { AddFlags (_src_prog, hook, _arg_id, args, do_return, return_value) {
        let library: &str = &hook.library;
        let library_basename: &str = library.rsplit('/').next().unwrap_or(library);
        let symbol: &str = &hook.symbol;
        let num_args = args.len();
        let flags = match (library_basename, symbol) {
            // Filesystem
            ("libc.so.6", "creat") |
            ("libc.so.6", "creat64") => {
                libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC
            },
            ("libc.so.6", "lchown") => {
                libc::AT_SYMLINK_NOFOLLOW
            },
            ("libc.so.6", "rmdir") => {
                libc::AT_REMOVEDIR
            },
            _ => 0
        } as usize;
        let position = match (library_basename, symbol) {
            // Filesystem
            ("libc.so.6", "creat") |
            ("libc.so.6", "creat64") => {
                if num_args == 3 {
                    2
                } else {
                    num_args
                }
            },
            _ => num_args
        } as usize;
        let new_arg = crate::common::db::ArgumentRow {
            hook: hook.id,
            parent: None,
            id: -1,
            position: position as i64,
            real: flags,
            datatype: String::from("IntegerSigned"),
            pointer: false,
            signed: true,
            variadic: false,
            array: false
        };
        args.insert(position, new_arg);
}}
