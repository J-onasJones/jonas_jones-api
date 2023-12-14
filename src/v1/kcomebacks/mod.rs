use warp::Filter;
use reqwest::Error;

pub fn get_kcomebacks_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("v1").and(warp::path("kcomebacks"))
    // - /v1/kcomebacks/last_update
    // - /v1/kcomebacks/start_update with token
    // - /v1/kcomebacks/upcoming/today?limit={0-50}&offset={n}
    // - /v1/kcomebacks/upcoming/week?limit={0-50}&offset={n}
    // - /v1/kcomebacks/upcoming/month?limit={0-50}&offset={n}
    // - /v1/kcomebacks/filter/id?id={n}
    // - /v1/kcomebacks/filter/daterange?start={date: YYYY-MM-DD}&end={date: YYYY-MM-DD}&limit={0-50}&offset={n}
    // - /v1/kcomebacks/filter/artist?artist={artist}&limit={0-50}&offset={n}
    // - /v1/kcomebacks/filter/first
    // - /v1/kcomebacks/filter/last
    // - /v1/kcomebacks/filter/title?title={title}&limit={0-50}&offset={n}
    // - /v1/kcomebacks/filter/type?type={type}&limit={0-50}&offset={n}
    // - /v1/kcomebacks/filter/gettypes

        .and(warp::path("last_update").and(warp::get()).and_then(last_update_handler)
        .or(warp::path("start_update").map(|| "Not implemented yet")))
}

// get json data from https://cdn.jonasjones.dev/api/kcomebacks/rkpop_data.json
async fn fetch_data() -> Result<serde_json::Value, Error> {
    let url = "https://cdn.jonasjones.dev/api/kcomebacks/rkpop_data.json";
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

async fn last_update_handler() -> Result<impl warp::Reply, warp::Rejection> {

    match last_update().await {
        Ok(last_update_value) => Ok(warp::reply::json(&last_update_value)),
        Err(_) => {
            #[derive(Debug)]
            struct InternalServerError;

            impl warp::reject::Reject for InternalServerError {}
            Err(warp::reject::custom(InternalServerError))
        }
    }
}

async fn last_update() -> Result<serde_json::Value, Error> {
    // get the value of last_update of the first element of the json that fetch_data() returns
    let last_update_value = fetch_data().await?.get(0).unwrap().get("last_update").unwrap().clone();
    return Ok(last_update_value);
}
