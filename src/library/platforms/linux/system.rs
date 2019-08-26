use libc::{c_int};

pub const EACCES: i32 = 13;

pub unsafe fn errno_location() -> *mut c_int {
    libc::__errno_location()
}

pub fn get_current_uid() -> u32 {
    unsafe { libc::getuid() }
}
