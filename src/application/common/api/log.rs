// Database
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use crate::common::db;

// POST /log/exec
pub fn log_exec(exec: db::LogExecObject, addr: Option<SocketAddr>) -> Result<impl warp::Reply, warp::Rejection> {
    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    if addr.unwrap().ip() != localhost {
        return Err(warp::reject::not_found());
    }
    // Input to this function is untrusted
    let conn: rusqlite::Connection = db::db_open();
    db::insert_exec(&conn, exec);
    return Ok(warp::reply());
}
