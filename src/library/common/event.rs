use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::library::common::http;

// TODO: Unify common objects among library and binary
#[derive(Deserialize, Serialize)]
struct LogExecObject {
    program: String,
    hash: String,
    uid: u32,
    ts: u64,
    success: bool
}

pub fn get_timestamp() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs()
}

pub fn send_exec_event(uid: u32, program: &str, hash: &str, success: bool) -> () {
    let ts = get_timestamp();
    //eprintln!("UID: {} Program: {} Hash: {} Unix TS: {} Permitted: {}", uid, program.to_string(), hash.to_string(), ts, success);
    let log = LogExecObject {
        program: program.to_string(),
        hash: hash.to_string(),
        uid: uid,
        ts: ts,
        success: success
    };
    if let Ok(_response) = http::post("http://127.0.0.1:11998/log/exec")
                                // Prevents denial of service
                                .with_timeout(1)
                                .with_json(&log)
                                .unwrap()
                                .send() {
        ()
    } else {
        eprintln!("Failed to communicate with WhiteBeam service");
    }
}
