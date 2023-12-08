use std::env;

use warp::Filter;

use crate::v1::get_v1_routes;
use crate::{Logger, parse_ip};


pub async fn serve() {
    let api_ip = env::var("API_IP").unwrap();
    let api_port = env::var("API_PORT").unwrap();

    Logger::info(&format!("Server started on {}:{}", api_ip, api_port));

    let socket_addr = parse_ip();

    // GET (any) => reply with return from handle_path
    let routes = get_v1_routes()
        .or(warp::any().map(|| warp::reply::with_status("Not Found", warp::http::StatusCode::NOT_FOUND)));



    warp::serve(routes)
        .run(socket_addr)
        .await;

    Logger::info("Server stopped");

}
