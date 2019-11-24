// Database
use crate::application::common::db;

// POST /log/exec
pub fn log_exec(exec: db::LogExecObject) -> impl warp::Reply {
    // TODO: Verify remote IP is 127.0.0.1
    // Input to this function is untrusted
    eprintln!("UID: {} Program: {} Hash: {} Unix TS: {} Permitted: {}", exec.uid, exec.program, exec.hash, exec.ts, exec.success);
    let conn: rusqlite::Connection = db::db_open();
    db::insert_exec(&conn, exec);
    return Ok(warp::reply());
}
