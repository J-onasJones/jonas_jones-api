pub fn internal_server_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("Internal Server Error"))
        .unwrap()
}

pub fn not_found_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .unwrap()
}

pub fn bad_request_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("Bad Request"))
        .unwrap()
}

pub fn unauthorized_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::from("Unauthorized"))
        .unwrap()
}

pub fn forbidden_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .body(Body::from("Forbidden"))
        .unwrap()
}

pub fn method_not_allowed_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::from("Method Not Allowed"))
        .unwrap()
}

pub fn not_acceptable_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_ACCEPTABLE)
        .body(Body::from("Not Acceptable"))
        .unwrap()
}

pub fn conflict_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::CONFLICT)
        .body(Body::from("Conflict"))
        .unwrap()
}

pub fn gone_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::GONE)
        .body(Body::from("Gone"))
        .unwrap()
}
