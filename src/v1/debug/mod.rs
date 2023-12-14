use warp::Filter;

pub fn get_debug_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("debug")
        .and(warp::path("headers").and(warp::header::headers_cloned().map(handle_with_headers)))
}

fn handle_with_headers(
    headers: warp::http::HeaderMap,
) -> String {
    // Format headers into a plain text string
    let headers_text = headers
        .iter()
        .map(|(name, value)| format!("{}: {}\n", name.as_str(), value.to_str().unwrap_or("")))
        .collect::<String>();

    // Respond with the plain text-formatted headers
    headers_text
}