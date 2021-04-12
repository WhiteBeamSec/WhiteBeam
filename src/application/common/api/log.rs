// TODO: Log failures
// Database
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use crate::common::db;

// POST /log
pub async fn log(log: db::LogObject, addr: Option<SocketAddr>) -> Result<impl warp::Reply, warp::Rejection> {
    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let remote_addr = match addr {
        Some(inetaddr)  => inetaddr.ip(),
        None => return Err(warp::reject::not_found())
    };
    if remote_addr != localhost {
        return Err(warp::reject::not_found());
    }
    // Input to this function is untrusted
    let conn: rusqlite::Connection = match db::db_open(false) {
        Ok(c) => c,
        Err(_) => return Err(warp::reject::not_found())
    };
    let log_level = match db::get_log_level(&conn) {
        Ok(l) => l,
        Err(_) => return Err(warp::reject::not_found())
    };
    if log.class >= log_level {
        let _res = db::insert_log(&conn, log);
    }
    return Ok(warp::reply());
}
