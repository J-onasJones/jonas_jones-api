mod builtin;

pub use builtin::get_builtin_routes as get_v1_builtin_routes;

pub fn get_v1_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    return get_v1_builtin_routes();
}