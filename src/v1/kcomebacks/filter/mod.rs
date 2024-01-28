use std::{collections::{HashMap, HashSet}, vec};

use chrono::NaiveDate;
use serde_json::{Value, json};
use warp::Filter;

use crate::{error_responses::{BadRequestError, InternalServerError}, v1::kcomebacks::{fetch_data, create_json_response, parse_item, Item as EntryItem}};

pub fn get_kcomebacks_filter_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("filter")
        .and((warp::path("id").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(filter_id_handler))
        .or(warp::path("getall").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(filter_getall_handler))
        .or(warp::path("daterange").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(filter_daterange_handler))
        .or(warp::path("artist").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(filter_artist_handler))
        .or(warp::path("first").and(warp::get()).and_then(filter_first_handler))
        .or(warp::path("last").and(warp::get()).and_then(filter_last_handler))
        .or(warp::path("title").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(filter_title_handler))
        .or(warp::path("type").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(filter_type_handler))
        .or(warp::path("gettypes").and(warp::get()).and_then(filter_gettypes_handler))
        .or(warp::path("getinfo").and(warp::get().and_then(filter_getinfo_handler))))
}

async fn filter_id_handler(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Access the parameters from the HashMap
    let id = params.get("id").unwrap_or(&"".to_string()).to_string();

    // check if the parameters are valid
    if !id.parse::<i32>().is_ok() || id.parse::<i32>().unwrap() < 0 || id.is_empty() || params.len() > 1 {
        return Err(warp::reject::custom(BadRequestError));
    }

    let id = match id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => 1,
    };

    // fetch the data
    let data = fetch_data().await.unwrap();

    let item = data[id as usize + 1].clone();

    if item.is_null() {
        return Err(warp::reject::custom(BadRequestError));
    }

    
    // return the data
    Ok(warp::reply::json(&create_json_response(vec![&parse_item(&item)], 1)))
}

async fn filter_getall_handler(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Access the parameters from the HashMap
    let limit = params.get("limit").unwrap_or(&"".to_string()).to_string();
    let offset = params.get("offset").unwrap_or(&"".to_string()).to_string();

    // check if the parameters are valid
    if params.len() > 2
        || !limit.parse::<i32>().is_ok()
        || !offset.parse::<i32>().is_ok()
        || limit.parse::<i32>().unwrap() < 0
        || limit.parse::<i32>().unwrap() > 50
        || offset.parse::<i32>().unwrap() < 0
        || limit.is_empty()
        || offset.is_empty()
    {
        return Err(warp::reject::custom(BadRequestError));
    }

    // fetch the data
    let data = fetch_data().await.unwrap();

    // filter the data
    let filtered_data = match data {
        Value::Array(items) => {
            items
                .iter()
                .filter_map(|item| serde_json::from_value::<EntryItem>(item.clone()).ok())
                .collect()
        }
        _ => Vec::new(),
    };

    // get the total number of results
    let total_results = filtered_data.len();

    // apply the limit and offset
    let filtered_data = filtered_data.iter().skip(offset.parse::<usize>().unwrap()).take(limit.parse::<usize>().unwrap()).collect::<Vec<_>>();

    // return the data
    Ok(warp::reply::json(&create_json_response(filtered_data, total_results)))
}

pub async fn filter_daterange_handler(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Access the parameters from the HashMap
    let start = params.get("start").unwrap_or(&"".to_string()).to_string();
    let end = params.get("end").unwrap_or(&"".to_string()).to_string();
    let limit = params.get("limit").unwrap_or(&"".to_string()).to_string();
    let offset = params.get("offset").unwrap_or(&"".to_string()).to_string();

    // Parse the start and end dates into NaiveDate
    let start_date_parsed = NaiveDate::parse_from_str(&start, "%Y-%m-%d");
    let end_date_parsed = NaiveDate::parse_from_str(&end, "%Y-%m-%d");

    // check if the parameters are valid
    if !start_date_parsed.is_ok()
        || !end_date_parsed.is_ok()
        || start_date_parsed.unwrap() > end_date_parsed.unwrap()
        || start.is_empty()
        || end.is_empty()
        || params.len() > 4
        || !limit.parse::<i32>().is_ok()
        || !offset.parse::<i32>().is_ok()
        || limit.parse::<i32>().unwrap() < 0
        || limit.parse::<i32>().unwrap() > 50
        || offset.parse::<i32>().unwrap() < 0
        || limit.is_empty()
        || offset.is_empty()
    {
        return Err(warp::reject::custom(BadRequestError));
    }

    // fetch the data
    let data = fetch_data().await.unwrap();

    // filter the data
    let filtered_data = match (start_date_parsed, end_date_parsed) {
        (Ok(start), Ok(end)) => {
            match data {
                Value::Array(items) => {
                    items
                        .iter()
                        .filter_map(|item| serde_json::from_value::<EntryItem>(item.clone()).ok())
                        .filter(|item| {
                            if let Ok(item_date) = NaiveDate::parse_from_str(&item.date, "%Y-%m-%d") {
                                item_date >= start && item_date <= end
                            } else {
                                false
                            }
                        })
                        .collect()
                }
                _ => Vec::new(),
            }
        }
        _ => Vec::new(),
    };

    // get the total number of results
    let total_results = filtered_data.len();

    // apply the limit and offset
    let filtered_data = filtered_data.iter().skip(offset.parse::<usize>().unwrap()).take(limit.parse::<usize>().unwrap()).collect::<Vec<_>>();

    // return the data
    Ok(warp::reply::json(&create_json_response(filtered_data, total_results)))
}

async fn filter_artist_handler(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Access the parameters from the HashMap
    let artist = params.get("artist").unwrap_or(&"".to_string()).to_string();
    let limit = params.get("limit").unwrap_or(&"".to_string()).to_string();
    let offset = params.get("offset").unwrap_or(&"".to_string()).to_string();

    // check if the parameters are valid
    if artist.is_empty()
        || params.len() > 3
        || !limit.parse::<i32>().is_ok()
        || !offset.parse::<i32>().is_ok()
        || limit.parse::<i32>().unwrap() < 0
        || limit.parse::<i32>().unwrap() > 50
        || offset.parse::<i32>().unwrap() < 0
        || limit.is_empty() || offset.is_empty()
    {
        return Err(warp::reject::custom(BadRequestError));
    }

    // fetch the data
    let data = fetch_data().await.unwrap();

    // filter the data
    let filtered_data = match data {
        Value::Array(items) => {
            items
                .iter()
                .filter_map(|item| {
                    // Attempt to deserialize the item as an Item
                    serde_json::from_value::<EntryItem>(item.clone()).ok()
                })
                .filter(|item| item.artist.to_lowercase().contains(artist.to_lowercase().as_str()))
                .collect()
        }
        _ => Vec::new(),
    };

    // get the total number of results
    let total_results = filtered_data.len();

    // apply the limit and offset
    let filtered_data = filtered_data.iter().skip(offset.parse::<usize>().unwrap()).take(limit.parse::<usize>().unwrap()).collect::<Vec<_>>();

    // return the data
    Ok(warp::reply::json(&create_json_response(filtered_data, total_results)))
}

async fn filter_first_handler() -> Result<impl warp::Reply, warp::Rejection> {

    // fetch the data
    let item = fetch_data().await.unwrap()[1].clone();

    if item.is_null() {
        return Err(warp::reject::custom(InternalServerError));
    }

    // return the data
    Ok(warp::reply::json(&create_json_response(vec![&parse_item(&item)], 1)))
}

async fn filter_last_handler() -> Result<impl warp::Reply, warp::Rejection> {

    // fetch the data
    let data = fetch_data().await.unwrap();

    // filter the data
    let item = data[data.as_array().unwrap().len() - 1].clone();

    // return the data
    Ok(warp::reply::json(&create_json_response(vec![&parse_item(&item)], 1)))
}

async fn filter_title_handler(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Access the parameters from the HashMap
    let title = params.get("title").unwrap_or(&"".to_string()).to_string();
    let limit = params.get("limit").unwrap_or(&"".to_string()).to_string();
    let offset = params.get("offset").unwrap_or(&"".to_string()).to_string();

    // check if the parameters are valid
    if title.is_empty()
        || params.len() > 3
        || !limit.parse::<i32>().is_ok()
        || !offset.parse::<i32>().is_ok()
        || limit.parse::<i32>().unwrap() < 0
        || limit.parse::<i32>().unwrap() > 50
        || offset.parse::<i32>().unwrap() < 0
        || limit.is_empty()
        || offset.is_empty()
    {
        return Err(warp::reject::custom(BadRequestError));
    }

    // fetch the data
    let data = fetch_data().await.unwrap();

    // filter the data
    let filtered_data = match data {
        Value::Array(items) => {
            items
                .iter()
                .filter_map(|item| {
                    // Attempt to deserialize the item as an Item
                    serde_json::from_value::<EntryItem>(item.clone()).ok()
                })
                .filter(|item| item.title.to_lowercase().contains(title.to_lowercase().as_str()))
                .collect()
        }
        _ => Vec::new(),
    };

    // get the total number of results
    let total_results = filtered_data.len();

    // apply the limit and offset
    let filtered_data = filtered_data.iter().skip(offset.parse::<usize>().unwrap()).take(limit.parse::<usize>().unwrap()).collect::<Vec<_>>();

    // return the data
    Ok(warp::reply::json(&create_json_response(filtered_data, total_results)))
}

async fn filter_type_handler(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Access the parameters from the HashMap
    let type_ = params.get("type").unwrap_or(&"".to_string()).to_string();
    let limit = params.get("limit").unwrap_or(&"".to_string()).to_string();
    let offset = params.get("offset").unwrap_or(&"".to_string()).to_string();

    // check if the parameters are valid
    if type_.is_empty()
        || params.len() > 3
        || !limit.parse::<i32>().is_ok()
        || !offset.parse::<i32>().is_ok()
        || limit.parse::<i32>().unwrap() < 0
        || limit.parse::<i32>().unwrap() > 50
        || offset.parse::<i32>().unwrap() < 0
        || limit.is_empty()
        || offset.is_empty()
    {
        return Err(warp::reject::custom(BadRequestError));
    }

    // fetch the data
    let data = fetch_data().await.unwrap();

    // filter the data
    let filtered_data = match data {
        Value::Array(items) => {
            items
                .iter()
                .filter_map(|item| {
                    serde_json::from_value::<EntryItem>(item.clone()).ok()
                })
                .filter(|item| item.types.iter().any(|t| t == type_.as_str()))
                .collect()
        }
        _ => Vec::new(),
    };

    // get the total number of results
    let total_results = filtered_data.len();

    // apply the limit and offset
    let filtered_data = filtered_data.iter().skip(offset.parse::<usize>().unwrap()).take(limit.parse::<usize>().unwrap()).collect::<Vec<_>>();

    // return the data
    Ok(warp::reply::json(&create_json_response(filtered_data, total_results)))
}

async fn filter_gettypes_handler() -> Result<impl warp::Reply, warp::Rejection> {
    // fetch the data
    let data = fetch_data().await.unwrap();

    // filter the data
    let mut types: Vec<String> = Vec::new();
    for i in 1..data.as_array().unwrap().len() {
        for j in 0..data[i]["types"].as_array().unwrap().len() {
            types.push(data[i]["types"][j].as_str().unwrap().to_string());
        }
    }

    // remove duplicates
    types.sort();
    types.dedup();

    // get the total number of results
    let total_results = types.len();

    // json response
    let json_response = json!({
        "results": types,
        "total_results": total_results,
    });

    // return the data
    Ok(warp::reply::json(&json_response))
}

async fn filter_getinfo_handler() -> Result<impl warp::Reply, warp::Rejection> {

    // fetch the data
    let data = fetch_data().await.unwrap();

    // get the number of items
    let num_items = data.as_array().unwrap().len() - 1;

    // get the number of artists
    let mut artists: HashSet<String> = HashSet::new();

    for i in 1..data.as_array().unwrap().len() {
        artists.insert(data[i]["artist"].as_str().unwrap().to_string());
    }

    let num_artists = artists.len();

    // get the start and end dates
    let start_date = data[1]["date"].as_str().unwrap().to_string();
    let end_date = data[num_items]["date"].as_str().unwrap().to_string();

    // return the data
    Ok(warp::reply::json(&serde_json::json!({
        "num_items": num_items,
        "num_artists": num_artists,
        "start_date": start_date,
        "end_date": end_date
    })))
}