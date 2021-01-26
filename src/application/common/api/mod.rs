use warp::Filter;

// Endpoints
mod status;
mod log;
mod service;

pub async fn serve() {
    // GET /status
    let status_route = warp::get()
        .and(warp::path("status"))
        .and(warp::path::end())
        .map(status::status);

    // POST /log/exec {"program":"whoami","hash":"..","uid":1000,"ts":1566162863,"success":true}
    let log_exec_route = warp::post()
        .and(warp::path("log"))
        .and(warp::path("exec"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(warp::addr::remote())
        .and_then(log::log_exec);

    // GET /service/public_key
    let service_public_key_route = warp::get()
        .and(warp::path("service"))
        .and(warp::path("public_key"))
        .and(warp::path::end())
        .and_then(service::public_key);

    // POST /service/encrypted {"pubkey":"..","nonce":"..","ciphertext":".."}
    let service_encrypted_route = warp::post()
        .and(warp::path("service"))
        .and(warp::path("encrypted"))
        .and(warp::path::end())
        .and(warp::body::json())
        .map(service::encrypted);

    let routes = status_route
                .or(log_exec_route)
                .or(service_public_key_route)
                .or(service_encrypted_route);
    // TODO: Use ServicePort setting
    warp::serve(routes).run(([0, 0, 0, 0], 11998)).await;
}
