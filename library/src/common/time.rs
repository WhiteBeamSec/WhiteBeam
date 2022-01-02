use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> u32 {
        let start = SystemTime::now();
        let since_the_epoch = match start.duration_since(UNIX_EPOCH) {
            Ok(t) => t,
            Err(_e) => {
                // TODO: Log
                eprintln!("WhiteBeam: System clock went backwards");
                Duration::from_secs(0)
            }
        };
        since_the_epoch.as_secs() as u32
}
