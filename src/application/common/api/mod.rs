use warp::Filter;

pub fn serve() {
    let routes = warp::any().map(|| "Hello, World!");
    warp::serve(routes).run(([0, 0, 0, 0], 11998));
}
