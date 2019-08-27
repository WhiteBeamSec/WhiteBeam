use std::time::{SystemTime, UNIX_EPOCH};
use crate::library::common::http;

pub fn get_timestamp() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs()
}

pub fn send_exec_event(uid: u32, program: &str, hash: &str, success: bool) -> () {
    let ts = get_timestamp();
    eprintln!("UID: {} Program: {} Hash: {} Unix TS: {} Permitted: {}", uid, program.to_string(), hash.to_string(), ts, success);
    if let Ok(response) = http::get("http://127.0.0.1/").send() {
        println!("{}", response.body);
    }
}
