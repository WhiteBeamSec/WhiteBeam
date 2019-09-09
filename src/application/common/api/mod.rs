use warp::Filter;

// Database
mod db;

// POST /log/exec
fn log_exec(exec: db::LogExecObject) -> impl warp::Reply {
    // Input to this function is untrusted
    eprintln!("UID: {} Program: {} Hash: {} Unix TS: {} Permitted: {}", exec.uid, exec.program, exec.hash, exec.ts, exec.success);
    let conn: rusqlite::Connection = db::open();
    db::insert_exec(&conn, exec);
    return Ok(warp::reply());
}

pub fn serve() {
    // POST /log/exec {"program":"whoami","hash":"..","uid":1000,ts:1566162863}
    let routes = warp::post2()
        .and(warp::path("log"))
        .and(warp::path("exec"))
        .and(warp::path::end())
        .and(warp::body::json())
        .map(log_exec);

    warp::serve(routes).run(([0, 0, 0, 0], 11998));
}
