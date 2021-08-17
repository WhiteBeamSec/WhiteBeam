#[allow(dead_code)]
fn build_library_path(library_basename: &str) -> String {
    #[cfg(target_os = "linux")]
    let lib_path = format!("/lib/{}-linux-gnu/{}", crate::common::db::get_setting(String::from("SystemArchitecture")), library_basename);
    #[cfg(not(target_os = "linux"))]
    unimplemented!("WhiteBeam: This platform is not supported by the RedirectFunction action");
    lib_path
}

pub fn get_redirected_function(library: &str, symbol: &str) -> (String, String) {
    // Use respective 64 bit functions? ftruncate64, openat64, __openat64_2
    let library_basename: &str = library.rsplit('/').next().unwrap_or(library);
    match (library_basename, symbol) {
        // Execution
        ("libc.so.6", "execl") |
        ("libc.so.6", "execle") |
        ("libc.so.6", "execlp") |
        ("libc.so.6", "execv") |
        ("libc.so.6", "execvp") |
        ("libc.so.6", "execvpe") => {
            (String::from(library), String::from("execve"))
        },
        // Filesystem
        ("libc.so.6", "truncate") |
        ("libc.so.6", "truncate64") => {
            (String::from(library), String::from("ftruncate"))
        },
        ("libc.so.6", "fopen") |
        ("libc.so.6", "fopen64") => {
            (String::from(library), String::from("fdopen"))
        },
        ("libc.so.6", "symlink") => {
            (String::from(library), String::from("symlinkat"))
        },
        ("libc.so.6", "mkdir") => {
            (String::from(library), String::from("mkdirat"))
        },
        ("libc.so.6", "unlink") |
        ("libc.so.6", "rmdir") => {
            (String::from(library), String::from("unlinkat"))
        },
        ("libc.so.6", "link") => {
            (String::from(library), String::from("linkat"))
        },
        ("libc.so.6", "rename") => {
            (String::from(library), String::from("renameat"))
        },
        ("libc.so.6", "chown") |
        ("libc.so.6", "lchown") => {
            (String::from(library), String::from("fchownat"))
        },
        ("libc.so.6", "chmod") => {
            (String::from(library), String::from("fchmodat"))
        },
        ("libc.so.6", "creat") |
        ("libc.so.6", "open") |
        ("libc.so.6", "creat64") |
        ("libc.so.6", "open64") => {
            (String::from(library), String::from("openat"))
        },
        ("libc.so.6", "mknod") => {
            (String::from(library), String::from("mknodat"))
        },
        ("libc.so.6", "__open") |
        ("libc.so.6", "__open_2") |
        ("libc.so.6", "__open64") |
        ("libc.so.6", "__open64_2") => {
            (String::from(library), String::from("__openat_2"))
        },
        ("libc.so.6", "__xmknod") => {
            (String::from(library), String::from("__xmknodat"))
        },
        _ => (String::from(library), String::from(symbol))
    }
}

build_action! { RedirectFunction (_src_prog, hook, _arg_id, args, do_return, return_value) {
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
        let redirected_function = get_redirected_function(library, symbol);
        hook.library = redirected_function.0;
        hook.symbol = redirected_function.1;
}}