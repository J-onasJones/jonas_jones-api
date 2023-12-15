use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorMessage {
    pub(crate) code: u16,
    pub(crate) message: String,
}

#[derive(Debug)]
pub struct InternalServerError;
impl warp::reject::Reject for InternalServerError {}

#[derive(Debug)]
pub struct BadRequestError;
impl warp::reject::Reject for BadRequestError {}

#[derive(Debug)]
pub struct NotFoundError;
impl warp::reject::Reject for NotFoundError {}

#[derive(Debug)]
pub struct UnauthorizedError;
impl warp::reject::Reject for UnauthorizedError {}

#[derive(Debug)]
pub struct ForbiddenError;
impl warp::reject::Reject for ForbiddenError {}

