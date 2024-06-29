use warp::Filter;

pub fn get_auth_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("v1").and(warp::path("auth"))
        .and((warp::path("requestsession").and(warp))
        .or(warp::path("login")))
}