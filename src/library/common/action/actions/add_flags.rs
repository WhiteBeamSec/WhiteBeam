#[macro_use]
build_action! { AddFlags (_src_prog, hook, _arg_id, args, do_return, return_value) {
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
        let num_args = args.len();
        let flags = match (library, symbol) {
            // Filesystem
            ("/lib/x86_64-linux-gnu/libc.so.6", "creat") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "creat64") => {
                libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "lchown") => {
                libc::AT_SYMLINK_NOFOLLOW
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "rmdir") => {
                libc::AT_REMOVEDIR
            },
            _ => 0
        } as usize;
        let position = match (library, symbol) {
            // Filesystem
            ("/lib/x86_64-linux-gnu/libc.so.6", "creat") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "creat64") => {
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
