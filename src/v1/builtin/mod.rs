// pub fn help() -> warp::reply::Response {

//     return warp::reply::Reply::with_header(
//         warp::reply::html(""),
//         "Content-Type",
//         "text/html",
//     );
// }

// create a function help() that returns a response with a ststus code of 200 and a body of "help"


use warp::Filter;
pub fn get_builtin_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("v1")
        .and((warp::path("help").map(|| "help"))
        .or(warp::path("ping").map(|| "pong"))
        .or(warp::path("version").map(|| warp::reply::json(&[option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")]))))
}
