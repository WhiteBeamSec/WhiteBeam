#[macro_use]
build_action! { RedirectFunction (_src_prog, hook, _arg_id, args, do_return, return_value) {
        let library: &str = &hook.library;
        let library_basename: &str = library.rsplit('/').next().unwrap_or(library);
        let symbol: &str = &hook.symbol;
        hook.symbol = match (library_basename, symbol) {
            // Execution
            ("libc.so.6", "execl") |
            ("libc.so.6", "execle") |
            ("libc.so.6", "execlp") |
            ("libc.so.6", "execv") |
            ("libc.so.6", "execvp") |
            ("libc.so.6", "execvpe") => {
                String::from("execve")
            },
            // Filesystem
            ("libc.so.6", "truncate") |
            ("libc.so.6", "truncate64") => {
                String::from("ftruncate")
            },
            ("libc.so.6", "fopen") |
            ("libc.so.6", "fopen64") => {
                String::from("fdopen")
            },
            ("libc.so.6", "symlink") => {
                String::from("symlinkat")
            },
            ("libc.so.6", "mkdir") => {
                String::from("mkdirat")
            },
            ("libc.so.6", "unlink") |
            ("libc.so.6", "rmdir") => {
                String::from("unlinkat")
            },
            ("libc.so.6", "link") => {
                String::from("linkat")
            },
            ("libc.so.6", "rename") => {
                String::from("renameat")
            },
            ("libc.so.6", "chown") |
            ("libc.so.6", "lchown") => {
                String::from("fchownat")
            },
            ("libc.so.6", "chmod") => {
                String::from("fchmodat")
            },
            ("libc.so.6", "creat") |
            ("libc.so.6", "open") |
            ("libc.so.6", "creat64") |
            ("libc.so.6", "open64") => {
                String::from("openat")
            },
            ("libc.so.6", "mknod") => {
                String::from("mknodat")
            },
            ("libc.so.6", "__open") |
            ("libc.so.6", "__open_2") |
            ("libc.so.6", "__open64") |
            ("libc.so.6", "__open64_2") => {
                String::from("__openat_2")
            },
            ("libc.so.6", "__xmknod") => {
                String::from("__xmknodat")
            },
            _ => { unimplemented!("WhiteBeam: The '{}' symbol (from {}) is not supported by the RedirectFunction action", symbol, library) }
        };
}}
