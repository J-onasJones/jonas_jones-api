mod minecraft;

use minecraft::get_minecraft_paths;

pub fn get_updates_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_minecraft_paths()
}