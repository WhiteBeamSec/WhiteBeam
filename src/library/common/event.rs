#[cfg(target_os = "windows")]
use crate::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::platforms::macos as platform;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::common::http;

#[derive(Deserialize, Serialize)]
struct LogExecObject {
    program: String,
    hash: String,
    uid: u32,
    ts: u32,
    success: bool
}

pub fn get_timestamp() -> u32 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs() as u32
}

pub fn send_exec_event(uid: u32, program: &OsStr, hash: &str, success: bool) -> () {
    let program_string = program.to_string_lossy().to_string();
    let ts = get_timestamp();
    let log = LogExecObject {
        program: program_string,
        hash: hash.to_string(),
        uid: uid,
        ts: ts,
        success: success
    };
    // https://github.com/noproto/WhiteBeam/blob/master/src/library/common/whitelist.rs#L40
    if platform::get_uptime().unwrap().as_secs() < (60*5) {
        return;
    }
    if let Ok(_response) = http::post("http://127.0.0.1:11998/log/exec")
                                // Prevents denial of service
                                .with_timeout(1)
                                .with_json(&log)
                                .unwrap()
                                .send() {
        ()
    } else {
        eprintln!("WhiteBeam: Failed to communicate with WhiteBeam service");
    }
}
