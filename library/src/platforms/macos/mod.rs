// Load OS-specific modules
// TODO: DYLD_INSERT_LIBRARIES globally

use std::path::PathBuf;

pub fn get_data_file_path(data_file: &str) -> PathBuf {
    #[cfg(feature = "whitelist_test")]
    let data_path: String = format!("{}/target/aarch64-unknown-linux-gnu/debug/examples/data/", env!("PWD"));
    #[cfg(not(feature = "whitelist_test"))]
    let data_path: String = String::from("/Applications/WhiteBeam/data/");
    let data_file_path = data_path + data_file;
    PathBuf::from(data_file_path)
}

pub fn get_realtime_file_path(realtime_file: &str) -> PathBuf {
    #[cfg(feature = "whitelist_test")]
    let realtime_path: String = format!("{}/target/aarch64-unknown-linux-gnu/debug/examples/realtime/", env!("PWD"));
    #[cfg(not(feature = "whitelist_test"))]
    let realtime_path: String = String::from("/Applications/WhiteBeam/realtime/");
    let realtime_file_path = realtime_path + realtime_file;
    PathBuf::from(realtime_file_path)
}

#[link(name = "pthread")]
extern "C" {
    fn pthread_threadid_np(thread: libc::pthread_t, thread_id: *mut libc::uint64_t) -> libc::c_int;
}

pub fn gettid() -> u64 {
    let mut result = 0;
    unsafe {let _ = pthread_threadid_np(0, &mut result); }
    result
}