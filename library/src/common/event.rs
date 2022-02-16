use crate::common::db;

pub fn send_log_event(class: i32, mut log: String) {
    // TODO: Multiplatform support
    #[cfg(feature = "whitelist_test")]
    return;
    let log_severity = match db::get_setting(String::from("LogSeverity")).parse::<i64>() {
        Ok(severity) => {
            match severity {
                // Optimized by the compiler
                0 => libc::LOG_EMERG,
                1 => libc::LOG_ALERT,
                2 => libc::LOG_CRIT,
                3 => libc::LOG_ERR,
                4 => libc::LOG_WARNING,
                5 => libc::LOG_NOTICE,
                6 => libc::LOG_INFO,
                7 => libc::LOG_DEBUG,
                // TODO: Log errors
                _ => libc::LOG_EMERG
            }
        },
        // TODO: Log errors
        Err(_) => libc::LOG_EMERG // 0
    };
    let log_facility = match db::get_setting(String::from("LogFacility")).parse::<i64>() {
        Ok(facility) => {
            match facility {
                // Optimized by the compiler
                0 => libc::LOG_KERN,
                1 => libc::LOG_USER,
                2 => libc::LOG_MAIL,
                3 => libc::LOG_DAEMON,
                4 => libc::LOG_AUTH,
                5 => libc::LOG_SYSLOG,
                6 => libc::LOG_LPR,
                7 => libc::LOG_NEWS,
                8 => libc::LOG_UUCP,
                9 => libc::LOG_CRON,
                10 => libc::LOG_AUTHPRIV,
                11 => libc::LOG_FTP,
                16 => libc::LOG_LOCAL0,
                17 => libc::LOG_LOCAL1,
                18 => libc::LOG_LOCAL2,
                19 => libc::LOG_LOCAL3,
                20 => libc::LOG_LOCAL4,
                21 => libc::LOG_LOCAL5,
                22 => libc::LOG_LOCAL6,
                23 => libc::LOG_LOCAL7,
                // TODO: Log errors
                _ => libc::LOG_LOCAL0 // 16
            }
        },
        // TODO: Log errors
        Err(_) => libc::LOG_LOCAL0 // 16
    };
    if log_severity < class {
        return;
    }
    // Null terminate log
    log.push('\0');
    unsafe {
        libc::openlog("WhiteBeam\0".as_ptr() as *const libc::c_char, libc::LOG_PID | libc::LOG_NDELAY, log_facility);
        libc::syslog(class, "%s\0".as_ptr() as *const libc::c_char, log.as_ptr() as *const libc::c_char);
        libc::closelog();
    }
}