#[macro_use]
build_action! { RedirectFunction (_src_prog, hook, _arg_id, args, do_return, return_value) {
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
        hook.symbol = match (library, symbol) {
            // Execution
            ("/lib/x86_64-linux-gnu/libc.so.6", "execl") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "execle") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "execlp") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "execv") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "execve") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "execvp") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "execvpe") => {
                String::from("fexecve")
            },
            // Filesystem
            ("/lib/x86_64-linux-gnu/libc.so.6", "truncate") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "truncate64") => {
                String::from("ftruncate")
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "fopen") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "fopen64") => {
                String::from("fdopen")
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "symlink") => {
                String::from("symlinkat")
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "unlink") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "rmdir") => {
                String::from("unlinkat")
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "link") => {
                String::from("linkat")
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "rename") => {
                String::from("renameat")
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "chown") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "lchown") => {
                String::from("fchownat")
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "chmod") => {
                String::from("fchmodat")
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "creat") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "open") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "creat64") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "open64") => {
                String::from("openat")
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "mknod") => {
                String::from("mknodat")
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "__open") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "__open_2") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "__open64") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "__open64_2") => {
                String::from("__openat_2")
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "__xmknod") => {
                String::from("__xmknodat")
            },
            _ => { unimplemented!("WhiteBeam: The '{}' symbol (from {}) is not supported by the RedirectFunction action", symbol, library) }
        };
}}
