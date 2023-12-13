mod builtin;
mod kcomebacks;

pub use builtin::get_builtin_routes as get_v1_builtin_routes;
pub use kcomebacks::get_kcomebacks_routes as get_v1_kcomebacks_routes;

use warp::Filter;

pub fn get_v1_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    return get_v1_builtin_routes()
        .or(get_v1_kcomebacks_routes());
}