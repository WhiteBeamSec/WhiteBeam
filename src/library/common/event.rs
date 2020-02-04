#[cfg(target_os = "windows")]
use crate::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::platforms::macos as platform;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use crate::common::http;
use crate::common::time;

#[derive(Deserialize, Serialize)]
struct LogExecObject {
    program: String,
    hash: String,
    uid: u32,
    ts: u32,
    success: bool
}

fn get_timeout() -> u64 {
    // Prevents denial of service
    1
}

pub fn send_exec_event(uid: u32, program: &OsStr, hash: &str, success: bool) {
    let program_string = program.to_string_lossy().to_string();
    let ts = time::get_timestamp();
    let log = LogExecObject {
        program: program_string,
        hash: hash.to_string(),
        uid: uid,
        ts: ts,
        success: success
    };
    if cfg!(feature = "whitelist_test") {
        return;
    }
    // https://github.com/WhiteBeamSec/WhiteBeam/blob/master/src/library/common/whitelist.rs#L59
    match platform::get_uptime() {
        Ok(uptime) => {
            if uptime.as_secs() < (60*5) {
                return;
            }
        },
        Err(e) => eprintln!("WhiteBeam: {}", e)
    };
    let request = match http::post("http://127.0.0.1:11998/log/exec")
                              .with_timeout(get_timeout())
                              .with_json(&log) {
                                  Ok(json_data) => json_data,
                                  Err(_e) => {
                                      eprintln!("WhiteBeam: Failed to serialize JSON");
                                      return;
                                  }
    };
    match request.send() {
        Ok(_response) => (),
        Err(_e) => eprintln!("WhiteBeam: Failed to communicate with WhiteBeam service")
    }
}
