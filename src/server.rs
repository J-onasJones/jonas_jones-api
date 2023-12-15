use std::convert::Infallible;
use std::env;

use reqwest::StatusCode;
use warp::Filter;
use warp::reply::Reply;

use crate::error_responses::{ErrorMessage, InternalServerError, BadRequestError, NotFoundError};
use crate::v1::get_v1_routes;
use crate::{Logger, parse_ip};


pub async fn serve() {
    let api_ip = env::var("API_IP").unwrap();
    let api_port = env::var("API_PORT").unwrap();

    Logger::info(&format!("Server started on {}:{}", api_ip, api_port));

    let socket_addr = parse_ip();

    // GET (any) => reply with return from handle_path
    let routes = get_v1_routes()
        .recover(handle_rejection);

    
    async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, Infallible> {
        let (code, message) = if err.find::<InternalServerError>().is_some() {
            (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
        } else if err.find::<BadRequestError>().is_some() {
            (StatusCode::BAD_REQUEST, "BAD_REQUEST")
        } else if err.is_not_found() || err.find::<NotFoundError>().is_some() {
            (StatusCode::NOT_FOUND, "NOT_FOUND")
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR") // Default case
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
