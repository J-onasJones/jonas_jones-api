mod builtin;
mod debug;
mod kcomebacks;
mod projects;
mod run;

pub use run::setup as run_setup;
pub use run::run_kcomebacks_command;
pub use run::run_projects_command;
pub use run::run_likedsongs_command;

pub use builtin::get_builtin_routes as get_v1_builtin_routes;
pub use debug::get_debug_routes as get_v1_debug_routes;
pub use kcomebacks::get_kcomebacks_routes as get_v1_kcomebacks_routes;
pub use projects::get_project_routes as get_v1_project_routes;
pub use run::get_run_routes as get_v1_updates_routes;

use warp::Filter;

pub fn get_v1_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    return get_v1_builtin_routes()
        .or(get_v1_debug_routes())
        .or(get_v1_kcomebacks_routes())
        .or(get_v1_project_routes())
        .or(get_v1_updates_routes());
}