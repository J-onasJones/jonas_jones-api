use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use warp::Filter;
use reqwest::Error;

mod filter;
mod upcoming;

use filter::get_kcomebacks_filter_routes;
use upcoming::get_kcomebacks_upcoming_routes;
use crate::error_responses::{InternalServerError, BadRequestError, NotFoundError};

pub fn get_kcomebacks_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("v1").and(warp::path("kcomebacks"))

        .and(warp::path("last_update").and(warp::get()).and_then(last_update)
        .or(warp::path("start_update").map(|| "Not implemented yet"))
        .or(get_kcomebacks_upcoming_routes())
        .or(get_kcomebacks_filter_routes())
        )
}

// get json data from https://cdn.jonasjones.dev/api/kcomebacks/rkpop_data.json
pub async fn fetch_data() -> Result<serde_json::Value, Error> {
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

async fn last_update() -> Result<impl warp::Reply, warp::Rejection> {

    // get the value of last_update of the first element of the json that fetch_data() returns
    let last_update_value = fetch_data().await.unwrap()[0]["last_update"].clone();

    // get the value from last_update_value and return it as a json if it's Ok, otherwise return an InternalServerError
    match last_update_value {
        serde_json::Value::String(last_update) => Ok(warp::reply::json(&last_update)),
        _ => Err(warp::reject::custom(InternalServerError)),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    artist: String,
    date: String,
    #[serde(default)]
    links: Vec<String>,
    time: String,
    title: String,
    types: Vec<String>,
}

pub fn create_json_response(items: Vec<&Item>, total_results: usize) -> Value {
    // Serialize the vector of items to a JSON array
    let results_array: Vec<Value> = items.into_iter().map(|item| json!(item)).collect();

    // Build the final JSON object with "results" and "total_results" fields
    let json_response = json!({
        "results": results_array,
        "total_results": total_results,
    });

    json_response
}

pub fn parse_item(item: &Value) -> Item {
    // Parse the item into a struct
    let item: Item = serde_json::from_value(item.clone()).unwrap();

    // Return the parsed item
    item
}
