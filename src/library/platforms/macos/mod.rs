// Load OS-specific modules
// TODO: DYLD_INSERT_LIBRARIES

use std::{mem,
          path::Path,
          time::Duration};

pub fn get_cache_file() -> &'static Path {
    Path::new("/Applications/WhiteBeam/data/cache.json")
}

pub fn get_uptime() -> Result<Duration, String> {
    let mut request = [libc::CTL_KERN, libc::KERN_BOOTTIME];
    let mut boottime: libc::timeval = unsafe { mem::zeroed() };
    let mut size: libc::size_t = mem::size_of_val(&boottime) as libc::size_t;
    let ret = unsafe {
        libc::sysctl(
            &mut request[0],
            2,
            &mut boottime as *mut libc::timeval as *mut libc::c_void,
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };
    if ret == 0 {
        Ok((time::now().to_timespec() - time::Timespec::new(boottime.tv_sec, boottime.tv_usec * 1000)))
    } else {
        Err("sysctl() failed".to_string())
    }
}
