pub fn load_sql(sql: &str) {
    use std::io::Write;
    // TODO: Cross platform
    let bin_path: std::path::PathBuf = std::path::PathBuf::from(format!("{}/target/release/whitebeam", env!("PWD")));
    assert!(bin_path.exists(), "WhiteBeam: whitebeam could not be found");
    let mut load_command = std::process::Command::new(bin_path)
            .args(&["--load", "-"])
            .env("WB_AUTH", "test")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .spawn().expect("Failed to execute process");
    let mut stdin = load_command.stdin.take().expect("Failed to capture stdin");
    write!(stdin, "{}", sql).expect("Failed to write to stdin");
    drop(stdin);
    match load_command.try_wait() {
        Ok(Some(_status)) => {},
        Ok(None) => {
            let _res = load_command.wait();
        },
        Err(_e) => {}
    }
}

pub fn toggle_hook(symbol: &str, enabled: bool) {
    assert!(symbol.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'));
    // TODO: Cross platform
    let sql = String::from(format!("UPDATE Hook SET enabled = {} WHERE symbol = '{}';", enabled, symbol));
    load_sql(&sql);
}

pub fn is_hooked(library: &str, symbol: &str) -> bool {
    let is_hooked_addr: usize = unsafe { libc::dlsym(libc::RTLD_DEFAULT, "is_hooked\0".as_ptr() as *const libc::c_char) } as usize;
    assert!(is_hooked_addr != 0, "WhiteBeam: is_hooked not found in libwhitebeam.so, consider running: cargo run build library-test");
    let is_hooked_fn: unsafe extern "C" fn(library: *const libc::c_char, symbol: *const libc::c_char) -> libc::c_int = unsafe { std::mem::transmute(is_hooked_addr) };
    let mut library_string = String::from(library);
    library_string.push('\0');
    let mut symbol_string = String::from(symbol);
    symbol_string.push('\0');
    let is_hooked_result: libc::c_int = unsafe { is_hooked_fn(library_string.as_ptr() as *const libc::c_char, symbol_string.as_ptr() as *const libc::c_char) };
    return is_hooked_result == 1;
}