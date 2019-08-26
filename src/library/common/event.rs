use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use hyper::Client;
use hyper::rt::{self, Future, Stream};

pub fn get_timestamp() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs()
}

// TODO: Segfaults bash? Need a way to work around this..
fn get() -> impl Future<Item=(), Error=()> {
    let client = Client::new();
    let uri = "http://127.0.0.1".parse().unwrap();
    client
        .get(uri)
        .and_then(|res| {
            res.into_body().for_each(|chunk| {
                io::stdout().write_all(&chunk)
                    .map_err(|e| panic!("stdout error: {}", e))
            })
        })
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
}

pub fn send_exec_event(uid: u32, program: &str, hash: &str, success: bool) -> () {
    let ts = get_timestamp();
    eprintln!("UID: {} Program: {} Hash: {} Unix TS: {} Permitted: {}", uid, program, hash, ts, success);
    rt::run(get());
    //let new_post = LogExecObject {
    //    program: program.to_string(),
    //    hash: hash.to_string(),
    //    uid: uid,
    //    ts: ts,
    //    success: success
    //};
}
