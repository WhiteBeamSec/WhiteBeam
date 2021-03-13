#[macro_use]
build_action! { VerifyCanWrite (_src_prog, hook, arg_id, args, do_return, return_value) {
        // https://docs.rs/glob/0.2.11/glob/struct.Pattern.html
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
        let dirfd = match (library, symbol) {
            ("/lib/x86_64-linux-gnu/libc.so.6", "fopen") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "fopen64") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "truncate") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "fchmod") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "fchown") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "fdopen") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "ftruncate") => {
                // TODO: basename fd -> dirfd
            },
            _ => {
                // TODO: dirfd
            }
        };
}}
