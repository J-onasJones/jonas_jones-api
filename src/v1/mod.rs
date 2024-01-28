mod builtin;
mod debug;
mod kcomebacks;
mod projects;
mod update;

pub use builtin::get_builtin_routes as get_v1_builtin_routes;
pub use debug::get_debug_routes as get_v1_debug_routes;
pub use kcomebacks::get_kcomebacks_routes as get_v1_kcomebacks_routes;
pub use projects::get_project_routes as get_v1_project_routes;
pub use update::get_update_routes as get_v1_updates_routes;

use warp::Filter;

pub fn get_v1_routes() -> impl warp::Filter<Extract = impl warp::Reply + warp::generic::Tuple, Error = warp::Rejection> + Clone {
    return get_v1_builtin_routes()
        .or(get_v1_debug_routes())
        .or(get_v1_kcomebacks_routes())
        .or(get_v1_project_routes())
        .or(get_v1_updates_routes());
}