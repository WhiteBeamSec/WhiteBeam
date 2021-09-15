use crate::common::db;
use syslog::{Facility, Formatter3164, Severity};

pub fn send_log_event(class: i64, log: String) {
    #[cfg(feature = "whitelist_test")]
    return;
    let log_severity = match db::get_setting(String::from("LogSeverity")).parse::<i64>() {
        Ok(severity) => {
            match severity {
                // Optimized by the compiler
                0 => Severity::LOG_EMERG,
                1 => Severity::LOG_ALERT,
                2 => Severity::LOG_CRIT,
                3 => Severity::LOG_ERR,
                4 => Severity::LOG_WARNING,
                5 => Severity::LOG_NOTICE,
                6 => Severity::LOG_INFO,
                7 => Severity::LOG_DEBUG,
                // TODO: Log errors
                _ => Severity::LOG_EMERG
            }
        },
        // TODO: Log errors
        Err(_) => Severity::LOG_EMERG // 0
    };
    let log_facility = match db::get_setting(String::from("LogFacility")).parse::<i64>() {
        Ok(facility) => {
            match facility {
                // Optimized by the compiler
                0 => Facility::LOG_KERN,
                1 => Facility::LOG_USER,
                2 => Facility::LOG_MAIL,
                3 => Facility::LOG_DAEMON,
                4 => Facility::LOG_AUTH,
                5 => Facility::LOG_SYSLOG,
                6 => Facility::LOG_LPR,
                7 => Facility::LOG_NEWS,
                8 => Facility::LOG_UUCP,
                9 => Facility::LOG_CRON,
                10 => Facility::LOG_AUTHPRIV,
                11 => Facility::LOG_FTP,
                16 => Facility::LOG_LOCAL0,
                17 => Facility::LOG_LOCAL1,
                18 => Facility::LOG_LOCAL2,
                19 => Facility::LOG_LOCAL3,
                20 => Facility::LOG_LOCAL4,
                21 => Facility::LOG_LOCAL5,
                22 => Facility::LOG_LOCAL6,
                23 => Facility::LOG_LOCAL7,
                // TODO: Log errors
                _ => Facility::LOG_LOCAL0 // 16
            }
        },
        // TODO: Log errors
        Err(_) => Facility::LOG_LOCAL0 // 16
    };
    if (log_severity as i64) < class {
        return;
    }
    let formatter = Formatter3164 {
        facility: log_facility,
        hostname: None,
        process: "WhiteBeam".into(),
        pid: unsafe { libc::getpid() },
    };
    // TODO: https://github.com/Geal/rust-syslog/issues/21
    // TODO: Does unix_connect need a timeout?
    if let Ok(mut writer) = syslog::unix(formatter) {
        let res = match log_severity {
            Severity::LOG_EMERG => writer.emerg(log),
            Severity::LOG_ALERT => writer.alert(log),
            Severity::LOG_CRIT => writer.crit(log),
            Severity::LOG_ERR => writer.err(log),
            Severity::LOG_WARNING => writer.warning(log),
            Severity::LOG_NOTICE => writer.notice(log),
            Severity::LOG_INFO => writer.info(log),
            Severity::LOG_DEBUG => writer.debug(log),
            _ => unreachable!("WhiteBeam: Lost track of environment")
        };
        if res.is_err() {
            eprintln!("WhiteBeam: Failed to connect to syslog");
        }
    } else {
        eprintln!("WhiteBeam: Failed to connect to syslog");
    }
}