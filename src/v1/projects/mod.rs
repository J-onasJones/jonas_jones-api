


pub fn get_project_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("v1").and(warp::path("projects"))

        .and(warp::path("last_update").and(warp::get()).and_then(last_update)
        .or(warp::path("start_update").map(|| "Not implemented yet"))
        .or(get_kcomebacks_upcoming_routes()))
}

// get json data from https://https://cdn.jonasjones.dev/api/projects/projects.json
pub async fn fetch_data() -> Result<serde_json::Value, Error> {
    let url = "https://https://cdn.jonasjones.dev/api/projects/projects.json";
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        // Parse the JSON response
        let json_data: serde_json::Value = response.json().await?;
        return Ok(json_data);
    } else {
        // Handle non-successful status codes
        Err(response.error_for_status().unwrap_err())
    }
}

async fn last_update() -> Result<impl warp::Reply, warp::Rejection> {

    // get the value of last_update of the first element of the json that fetch_data() returns
    let last_update_value = fetch_data().await.unwrap()[0]["last_update"].clone();

    // get the value from last_update_value and return it as a json if it's Ok, otherwise return an InternalServerError
    match last_update_value {
        serde_json::Value::String(last_update) => Ok(warp::reply::json(&last_update)),
        _ => Err(warp::reject::custom(InternalServerError)),
    }
}