use std::convert::Infallible;
use std::env;

use reqwest::StatusCode;
use warp::Filter;
use warp::reply::Reply;

use crate::error_responses::{ErrorMessage, InternalServerError, BadRequestError, NotFoundError, NotImplementedError};
use crate::v1::get_v1_routes;
use crate::{Logger, parse_ip};


pub async fn serve() {
    let api_ip = env::var("API_IP").unwrap();
    let api_port = env::var("API_PORT").unwrap();

    Logger::info(&format!("Server started on {}:{}", api_ip, api_port));

    let socket_addr = parse_ip();

    let favicon = warp::path("favicon.ico").and(warp::fs::file("./src/favicon.png"));

    // GET (any) => reply with return from handle_path
    let routes = favicon.or(get_v1_routes())
        .recover(handle_rejection);


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

        Ok(warp::reply::with_status(json, code))
    }


    warp::serve(routes)
        .run(socket_addr)
        .await;

    Logger::info("Server stopped");

}
