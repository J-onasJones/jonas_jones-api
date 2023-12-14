mod builtin;
mod debug;
mod kcomebacks;
mod updates;

pub use builtin::get_builtin_routes as get_v1_builtin_routes;
pub use debug::get_debug_routes as get_v1_debug_routes;
pub use kcomebacks::get_kcomebacks_routes as get_v1_kcomebacks_routes;
pub use updates::get_updates_routes as get_v1_updates_routes;

use warp::Filter;

pub fn get_v1_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    return get_v1_builtin_routes()
        .or(get_v1_debug_routes())
        .or(get_v1_kcomebacks_routes())
        .or(get_v1_updates_routes());
}