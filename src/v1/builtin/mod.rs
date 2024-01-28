use warp::Filter;

pub fn get_builtin_routes() -> impl warp::Filter<Extract = impl warp::Reply + warp::generic::Tuple, Error = warp::Rejection> + Clone {
    warp::path("v1")
        .and((warp::path("help").map(|| "Please refer to the wiki at https://wiki.jonasjones.dev/Api/"))
        .or(warp::path("ping").map(|| "pong"))
        .or(warp::path("version").map(|| warp::reply::json(&[option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")]))))
}
