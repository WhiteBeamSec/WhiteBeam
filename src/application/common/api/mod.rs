use warp::Filter;

// Database
mod db;

// GET /status
fn status() -> impl warp::Reply {
    return "OK";
}

// POST /log/exec
fn log_exec(exec: db::LogExecObject) -> impl warp::Reply {
    // TODO: Verify remote IP is 127.0.0.1
    // Input to this function is untrusted
    eprintln!("UID: {} Program: {} Hash: {} Unix TS: {} Permitted: {}", exec.uid, exec.program, exec.hash, exec.ts, exec.success);
    let conn: rusqlite::Connection = db::open();
    db::insert_exec(&conn, exec);
    return Ok(warp::reply());
}

pub fn serve() {
    // GET /status
    let status_route = warp::get2()
        .and(warp::path("status"))
        .and(warp::path::end())
        .map(status);

    // POST /log/exec {"program":"whoami","hash":"..","uid":1000,ts:1566162863}
    let log_exec_route = warp::post2()
        .and(warp::path("log"))
        .and(warp::path("exec"))
        .and(warp::path::end())
        .and(warp::body::json())
        .map(log_exec);

    let routes = status_route.or(log_exec_route);
    warp::serve(routes).run(([0, 0, 0, 0], 11998));
}
