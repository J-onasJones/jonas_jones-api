

pub fn get_kcomebacks_upcoming_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("upcoming")
        .and(warp::path("today").and(warp::get()).and_then(upcoming_today_handler))
        .or(warp::path("week").and(warp::get()).and_then(upcoming_week_handler))
        .or(warp::path("month").and(warp::get()).and_then(upcoming_month_handler))
}