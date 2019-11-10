use warp::Filter;

// Endpoints
mod status;
mod log;

pub fn serve() {
    // GET /status
    let status_route = warp::get2()
        .and(warp::path("status"))
        .and(warp::path::end())
        .map(status::status);

    // POST /log/exec {"program":"whoami","hash":"..","uid":1000,ts:1566162863}
    let log_exec_route = warp::post2()
        .and(warp::path("log"))
        .and(warp::path("exec"))
        .and(warp::path::end())
        .and(warp::body::json())
        .map(log::log_exec);

    let routes = status_route.or(log_exec_route);
    warp::serve(routes).run(([0, 0, 0, 0], 11998));
}
