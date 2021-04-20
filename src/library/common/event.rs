use serde::{Deserialize, Serialize};
use crate::common::{db, http, time};

#[derive(Deserialize, Serialize)]
struct LogObject {
    class: i64,
    log: String,
    ts: u32
}

pub enum LogClass {
    Off = 1,
    Error, // 2
    Warn, // 3
    Info, // 4
    Debug, // 5
    Trace // 6
}

fn get_timeout() -> u64 {
    // Prevents denial of service
    1
}

pub fn send_log_event(class: i64, log: String) {
    if cfg!(feature = "whitelist_test") {
        return;
    }
    let log_level: i64 = match db::get_setting(String::from("LogVerbosity")).parse() {
        Ok(level) => level,
        // TODO: Log errors
        Err(_) => 1
    };
    if log_level < class {
        return;
    }
    let ts = time::get_timestamp();
    let log_object = LogObject {
        class,
        log,
        ts
    };
    let service_port: i32 = match db::get_setting(String::from("ServicePort")).parse() {
        Ok(port) => port,
        // TODO: Log errors
        Err(_) => 11998
    };
    let request = match http::post(format!("http://127.0.0.1:{}/log", service_port))
                              .with_timeout(get_timeout())
                              .with_json(&log_object) {
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
