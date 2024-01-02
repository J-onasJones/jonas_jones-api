use warp::Filter;

pub fn get_mods_paths() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // any path that starts with /v1/updates/minecraft/mods/{modname}/{loadername}/{version} calls handle_path
    warp::path("v1").and(warp::path("updates")).and(warp::path("minecraft")).and(warp::path("mods"))

        .and(warp::path::param())
        .and(warp::path::param())
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::addr::remote())
        .map(handle_path)
}

fn handle_path(modname: String, loadername: String, version: String, remote_ip: Option<std::net::SocketAddr>) -> String {
    format!("modname: {}, loadername: {}, version: {}, IP: {}", modname, loadername, version, remote_ip.unwrap_or(std::net::SocketAddr::from(([0, 0, 0, 0], 0))).ip())
}

// fn handle_with_headers(
//     headers: warp::http::HeaderMap,
// ) -> String {
//     // Iterate through the headers and print them
//     for (name, value) in headers.iter() {
//         println!("Header: {}: {}", name, value.to_str().unwrap_or("Invalid UTF-8"));
//     }

//     // Respond with a message or perform other actions as needed
//     "Headers received".to_string()
// }
