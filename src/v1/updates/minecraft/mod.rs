mod mods;

use mods::get_mods_paths;

pub fn get_minecraft_paths() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_mods_paths()
}