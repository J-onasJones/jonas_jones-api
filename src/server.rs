use std::env;

use warp::Filter;

use crate::{Logger, parse_ip};


pub async fn serve() {
    let api_ip = env::var("API_IP").unwrap();
    let api_port = env::var("API_PORT").unwrap();

    Logger::info(&format!("Server started on {}:{}", api_ip, api_port));

    let socket_addr = parse_ip();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let root = warp::path::end()
        .map(|| warp::reply::html("<h1>jonas_jones-api!</h1>\n<p>this is a placeholder (hopefully)</p>"));

    let v1 = warp::path!("v1" / String)
        .map(|name| format!("Hello, {}!", name));


    let routes = hello.or(v1).or(root);

    warp::serve(routes)
        .run(socket_addr)
        .await;

    Logger::info("Server stopped");

}
