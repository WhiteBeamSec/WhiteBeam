use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use warp::Filter;

// Endpoints
mod status;
mod log;
mod service;

pub async fn serve(service_port: u16) {
    // GET /status
    let status_route = warp::get()
        .and(warp::path("status"))
        .and(warp::path::end())
        .map(status::status);

    // POST /log {"class":1,"desc":"..","ts":1566162863}
    let log_route = warp::post()
        .and(warp::path("log"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(warp::addr::remote())
        .and_then(log::log);

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
                .or(log_route)
                .or(service_public_key_route)
                .or(service_encrypted_route);
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), service_port);
    warp::serve(routes).run(socket).await;
}
