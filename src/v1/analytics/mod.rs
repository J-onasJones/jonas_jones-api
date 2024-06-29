use warp::Filter;

pub fn get_analytics_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("v1").and(warp::path("analytics"))
        .and((warp::path("logcdnrequest").map(|| "Please refer to the wiki at https://wiki.jonasjones.dev/Api/"))
        .or(warp::path("ping").map(|| "pong"))
        .or(warp::path("version").map(|| warp::reply::json(&[option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")]))))
}