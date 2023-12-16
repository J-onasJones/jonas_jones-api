use std::{collections::HashMap, ops::Add};

use warp::Filter;

use crate::{v1::kcomebacks::filter::filter_daterange_handler, error_responses::BadRequestError};

pub fn get_kcomebacks_upcoming_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("upcoming")
        .and((warp::path("today").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(upcoming_today_handler))
        .or(warp::path("week").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(upcoming_week_handler))
        .or(warp::path("month").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(upcoming_month_handler)))
}

async fn upcoming_today_handler(mut params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let start = today.clone();
    let end = today.clone();

    // add start and end to the params
    params.insert("start".to_string(), start);
    params.insert("end".to_string(), end);

    filter_daterange_handler(params).await
}

async fn upcoming_week_handler(mut params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let start = today.clone();
    let end = chrono::Local::now().add(chrono::Duration::days(7)).format("%Y-%m-%d").to_string();

    // add start and end to the params
    params.insert("start".to_string(), start);
    params.insert("end".to_string(), end);

    filter_daterange_handler(params).await
}

async fn upcoming_month_handler(mut params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let start = today.clone();
    let end = chrono::Local::now().add(chrono::Duration::days(30)).format("%Y-%m-%d").to_string();

    // add start and end to the params
    params.insert("start".to_string(), start);
    params.insert("end".to_string(), end);

    filter_daterange_handler(params).await
}