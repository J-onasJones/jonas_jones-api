use std::convert::Infallible;
use std::net::SocketAddr;
use std::env;

use lastfm::reqwest::StatusCode;
use warp::filters::path::FullPath;
use warp::Filter;
use warp::reply::Reply;

use crate::error_responses::{ErrorMessage, InternalServerError, BadRequestError, NotFoundError, NotImplementedError};
use crate::v1::get_v1_routes;
use crate::{parse_ip, request_logger, Logger};
use crate::iplookup::ip_lookup;


pub async fn serve() {
    let api_ip = env::var("API_IP").unwrap();
    let api_port = env::var("API_PORT").unwrap();

    Logger::info(&format!("Server started on {}:{}", api_ip, api_port));

    let socket_addr = parse_ip();

    let favicon = warp::path("favicon.ico").and(warp::fs::file("./src/favicon.png"));

    // /status => 200 OK
    let status = warp::path("status")
        .map(|| warp::reply());

    // Middleware filter to log request details
    let log_request = warp::any()
        .and(warp::method())
        .and(warp::path::full())
        .and(warp::addr::remote())
        .and(warp::header::optional::<String>("x-forwarded-for"))
        .map(|method, path: FullPath, addr: Option<SocketAddr>, fwd_for: Option<String>| {
            let client_ip = fwd_for.unwrap_or_else(|| addr.map(|a| a.ip().to_string()).unwrap_or_else(|| String::from("unknown")));
            let path_str = path.as_str();

            request_logger::log_request(&client_ip, path_str, method, "requests.json");
            Logger::info(&format!(" {} {} from {} ({})", method, path_str, ip_lookup(&client_ip), client_ip));
        });

    // GET (any) => reply with return from handle_path
    let routes = log_request
    .clone().untuple_one().and(favicon.or(status.or(get_v1_routes())
        .recover(handle_rejection)));


    async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, Infallible> {
        let (code, message) = if err.find::<InternalServerError>().is_some() {
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        } else if err.find::<BadRequestError>().is_some() {
            (StatusCode::BAD_REQUEST, "Bad Request")
        } else if err.is_not_found() || err.find::<NotFoundError>().is_some() {
            (StatusCode::NOT_FOUND, "Not Found")
        } else if err.find::<NotImplementedError>().is_some() {
            (StatusCode::NOT_IMPLEMENTED, "Not Implemented")
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, "Unhandled Rejection") // Default case
        };

        let json = warp::reply::json(&ErrorMessage {
            code: code.as_u16(),
            message: message.into(),
        });

        Ok(warp::reply::with_status(json, StatusCode::from_u16(code.as_u16()).unwrap()))
    }


    warp::serve(routes)
        .run(socket_addr)
        .await;

    Logger::info("Server stopped");

}
