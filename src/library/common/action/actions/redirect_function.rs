#[macro_use]
build_action! { RedirectFunction (_src_prog, hook, _arg_id, args, do_return, return_value) {
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
        match (library, symbol) {
            ("/lib/x86_64-linux-gnu/libc.so.6", "execl") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "execle") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "execlp") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "execv") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "execvp") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "execvpe") |
            ("/lib/x86_64-linux-gnu/libc.so.6", "fexecve") => {
                hook.symbol = String::from("execve");
            },
            _ => { unimplemented!("WhiteBeam: The '{}' symbol (from {}) is not supported by the RedirectFunction action", symbol, library) }
        };
}}
