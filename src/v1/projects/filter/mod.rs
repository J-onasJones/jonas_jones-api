use std::collections::{HashMap, HashSet};

use serde_json::{Value, json};
use warp::Filter;

use crate::{error_responses::BadRequestError, v1::projects::{fetch_data, create_json_response, Project as EntryProject}};

pub fn get_project_filter_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("filter")
        .and((warp::path("getall").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(filter_getall_handler))
        .or(warp::path("lastupdaterange").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(filter_lastupdaterange_handler))
        .or(warp::path("title").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(filter_title_handler))
        .or(warp::path("description").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(filter_description_handler))
        .or(warp::path("search").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(filter_search_handler))
        .or(warp::path("status").and(warp::get()).and(warp::query::<HashMap<String, String>>()).and_then(filter_status_handler))
        .or(warp::path("statuscolor").and(warp::get().and(warp::query::<HashMap<String, String>>()).and_then(filter_statuscolor_handler)))
        .or(warp::path("category").and(warp::get().and(warp::query::<HashMap<String, String>>()).and_then(filter_category_handler)))
        .or(warp::path("language").and(warp::get().and(warp::query::<HashMap<String, String>>()).and_then(filter_language_handler)))
        .or(warp::path("getlangs").and(warp::get().and_then(filter_getlangs_handler)))
        .or(warp::path("getstatuses").and(warp::get().and_then(filter_getstatuses_handler)))
        .or(warp::path("getcolors").and(warp::get().and_then(filter_getcolors_handler)))
        .or(warp::path("getcategories").and(warp::get().and_then(filter_getcategories_handler))))
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
    let filtered_data: Vec<EntryProject> = match data {
        Value::Array(items) => {
            items
                .iter()
                .filter_map(|item| serde_json::from_value::<EntryProject>(item.clone()).ok())
                .filter(|project| project.visible)
                .collect()
        }
        _ => Vec::new(),
    };


    // get the total number of results
    let total_results = filtered_data.len();

    // apply the limit and offset
    let filtered_data = filtered_data.iter().skip(offset.parse::<usize>().unwrap() + 1).take(limit.parse::<usize>().unwrap()).collect::<Vec<_>>();

    // return the data
    Ok(warp::reply::json(&create_json_response(filtered_data, total_results)))
}


async fn filter_lastupdaterange_handler(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Access the parameters from the HashMap
    let start = params.get("start").unwrap_or(&"".to_string()).to_string();
    let end = params.get("end").unwrap_or(&"".to_string()).to_string();
    let limit = params.get("limit").unwrap_or(&"".to_string()).to_string();
    let offset = params.get("offset").unwrap_or(&"".to_string()).to_string();

    // Parse the start and end dates (from unix time) to i64 integers
    let start = start.parse::<i64>().unwrap_or(-1);
    let end = end.parse::<i64>().unwrap_or(-1);

    // check if the parameters are valid
    if params.len() > 4
        || !limit.parse::<i32>().is_ok()
        || !offset.parse::<i32>().is_ok()
        || limit.parse::<i32>().unwrap() < 0
        || limit.parse::<i32>().unwrap() > 50
        || offset.parse::<i32>().unwrap() < 0
        || limit.is_empty()
        || offset.is_empty()
        || start == -1
        || end == -1
        || end < start
    {
        return Err(warp::reject::custom(BadRequestError));
    }

    // fetch the data
    let data = fetch_data().await.unwrap();

    // filter the data
    let filtered_data: Vec<EntryProject> = match data {
        Value::Array(items) => {
            items
                .iter()
                .filter_map(|item| serde_json::from_value::<EntryProject>(item.clone()).ok())
                .filter(|project| project.visible)
                .filter(|project| project.last_update >= start && project.last_update <= end)
                .collect()
        }
        _ => Vec::new(),
    };

    println!("{:?}", filtered_data);

    // get the total number of results
    let total_results = filtered_data.len();

    // apply the limit and offset
    let filtered_data = filtered_data.iter().skip(offset.parse::<usize>().unwrap()).take(limit.parse::<usize>().unwrap()).collect::<Vec<_>>();

    // return the data
    Ok(warp::reply::json(&create_json_response(filtered_data, total_results)))


}

async fn filter_title_handler(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Access the parameters from the HashMap
    let title = params.get("title").unwrap_or(&"".to_string()).to_string();
    let limit = params.get("limit").unwrap_or(&"".to_string()).to_string();
    let offset = params.get("offset").unwrap_or(&"".to_string()).to_string();

    // check if the parameters are valid
    if params.len() > 3
        || !limit.parse::<i32>().is_ok()
        || !offset.parse::<i32>().is_ok()
        || limit.parse::<i32>().unwrap() < 0
        || limit.parse::<i32>().unwrap() > 50
        || offset.parse::<i32>().unwrap() < 0
        || limit.is_empty()
        || offset.is_empty()
        || title.is_empty()
    {
        return Err(warp::reject::custom(BadRequestError));
    }

    // fetch the data
    let data = fetch_data().await.unwrap();

    // filter the data
    let filtered_data: Vec<EntryProject> = match data {
        Value::Array(items) => {
            items
                .iter()
                .filter_map(|item| serde_json::from_value::<EntryProject>(item.clone()).ok())
                .filter(|project| project.visible)
                .filter(|project| project.title.to_lowercase().contains(title.to_lowercase().as_str()))
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

async fn filter_description_handler(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Access the parameters from the HashMap
    let description = params.get("description").unwrap_or(&"".to_string()).to_string();
    let limit = params.get("limit").unwrap_or(&"".to_string()).to_string();
    let offset = params.get("offset").unwrap_or(&"".to_string()).to_string();

    // check if the parameters are valid
    if params.len() > 3
        || !limit.parse::<i32>().is_ok()
        || !offset.parse::<i32>().is_ok()
        || limit.parse::<i32>().unwrap() < 0
        || limit.parse::<i32>().unwrap() > 50
        || offset.parse::<i32>().unwrap() < 0
        || limit.is_empty()
        || offset.is_empty()
        || description.is_empty()
    {
        return Err(warp::reject::custom(BadRequestError));
    }

    // fetch the data
    let data = fetch_data().await.unwrap();

    // filter the data
    let filtered_data: Vec<EntryProject> = match data {
        Value::Array(items) => {
            items
                .iter()
                .filter_map(|item| serde_json::from_value::<EntryProject>(item.clone()).ok())
                .filter(|project| project.visible)
                .filter(|project| project.description.to_lowercase().contains(description.to_lowercase().as_str()))
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

async fn filter_search_handler(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Access the parameters from the HashMap
    let search = params.get("search").unwrap_or(&"".to_string()).to_string();
    let limit = params.get("limit").unwrap_or(&"".to_string()).to_string();
    let offset = params.get("offset").unwrap_or(&"".to_string()).to_string();

    // check if the parameters are valid
    if params.len() > 3
        || !limit.parse::<i32>().is_ok()
        || !offset.parse::<i32>().is_ok()
        || limit.parse::<i32>().unwrap() < 0
        || limit.parse::<i32>().unwrap() > 50
        || offset.parse::<i32>().unwrap() < 0
        || limit.is_empty()
        || offset.is_empty()
        || search.is_empty()
    {
        return Err(warp::reject::custom(BadRequestError));
    }

    // fetch the data
    let data = fetch_data().await.unwrap();

    // filter the data
    let filtered_data: Vec<EntryProject> = match data {
        Value::Array(items) => {
            items
                .iter()
                .filter_map(|item| serde_json::from_value::<EntryProject>(item.clone()).ok())
                .filter(|project| project.visible)
                .filter(|project| project.title.to_lowercase().contains(search.to_lowercase().as_str()) || project.description.to_lowercase().contains(search.to_lowercase().as_str()))
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

async fn filter_status_handler(params: HashMap<String, String>)  -> Result<impl warp::Reply, warp::Rejection> {
    // Access the parameres from the HashMap
    let status = params.get("status").unwrap_or(&"".to_string()).to_string();
    let limit = params.get("limit").unwrap_or(&"".to_string()).to_string();
    let offset = params.get("offset").unwrap_or(&"".to_string()).to_string();

    // check if the parameters are valid
    if status.is_empty()
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
    let filtered_data: Vec<EntryProject> = match data {
        Value::Array(items) => {
            items
                .iter()
                .filter_map(|item| serde_json::from_value::<EntryProject>(item.clone()).ok())
                .filter(|project| project.visible)
                .filter(|project| project.status == status)
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

async fn filter_statuscolor_handler(params: HashMap<String, String>)  -> Result<impl warp::Reply, warp::Rejection> {
    // Access the parameres from the HashMap
    let statuscolor = params.get("statuscolor").unwrap_or(&"".to_string()).to_string();
    let limit = params.get("limit").unwrap_or(&"".to_string()).to_string();
    let offset = params.get("offset").unwrap_or(&"".to_string()).to_string();

    // check if the parameters are valid
    if statuscolor.is_empty()
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
    let filtered_data: Vec<EntryProject> = match data {
        Value::Array(items) => {
            items
                .iter()
                .filter_map(|item| serde_json::from_value::<EntryProject>(item.clone()).ok())
                .filter(|project| project.visible)
                .filter(|project| project.statuscolor == statuscolor)
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

async fn filter_category_handler(params: HashMap<String, String>)  -> Result<impl warp::Reply, warp::Rejection> {
    // Access the parameres from the HashMap
    let category = params.get("category").unwrap_or(&"".to_string()).to_string();
    let limit = params.get("limit").unwrap_or(&"".to_string()).to_string();
    let offset = params.get("offset").unwrap_or(&"".to_string()).to_string();

    // check if the parameters are valid
    if category.is_empty()
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
    let filtered_data: Vec<EntryProject> = match data {
        Value::Array(items) => {
            items
                .iter()
                .filter_map(|item| serde_json::from_value::<EntryProject>(item.clone()).ok())
                .filter(|project| project.visible)
                .filter(|project| project.categories.iter().any(|cat| cat == category.as_str()))
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

async fn filter_language_handler(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Access the parameres from the HashMap
    let language = params.get("language").unwrap_or(&"".to_string()).to_string();
    let limit = params.get("limit").unwrap_or(&"".to_string()).to_string();
    let offset = params.get("offset").unwrap_or(&"".to_string()).to_string();

    // check if the parameters are valid
    if language.is_empty()
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
    let filtered_data: Vec<EntryProject> = match data {
        Value::Array(items) => {
            items
                .iter()
                .filter_map(|item| serde_json::from_value::<EntryProject>(item.clone()).ok())
                .filter(|project| project.visible)
                .filter(|project| project.languages.contains_key(&language))
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

async fn filter_getlangs_handler() -> Result<impl warp::Reply, warp::Rejection> {
    // fetch the data
    let data = fetch_data().await.unwrap();

    //filter the data
    let filtered_data: Vec<EntryProject> = match data {
        Value::Array(items) => {
            items
                .iter()
                .filter_map(|item| serde_json::from_value::<EntryProject>(item.clone()).ok())
                .filter(|project| project.visible)
                .collect()
        }
        _ => Vec::new(),
    };

    // filter the data
    let mut languages_set: HashSet<String> = HashSet::new();

    for project in filtered_data {
        for language in project.languages.keys() {
            languages_set.insert(language.clone());
        }
    }

    // get the total number of results
    let total_results = languages_set.len();

    // json response
    let json_response = json!({
        "results": languages_set,
        "total_results": total_results,
    });

    // return the data
    Ok(warp::reply::json(&json_response))
}

async fn filter_getstatuses_handler() -> Result<impl warp::Reply, warp::Rejection> {
    // fetch the data
    let data = fetch_data().await.unwrap();

    let mut all_statuses: Vec<String> = Vec::new();
    for i in 1..data.as_array().unwrap().len() {
        if !data[i]["status"].as_str().unwrap().to_string().is_empty() {
            all_statuses.push(data[i]["status"].as_str().unwrap().to_string());
        }
    }

    // remove duplicates
    all_statuses.sort();
    all_statuses.dedup();

    // get the total number of results
    let total_results = all_statuses.len();

    // json response
    let json_response = json!({
        "results": all_statuses,
        "total_results": total_results,
    });

    Ok(warp::reply::json(&json_response))
}

async fn filter_getcolors_handler() -> Result<impl warp::Reply, warp::Rejection> {
    // fetch the data
    let data = fetch_data().await.unwrap();

    let mut all_colors: Vec<String> = Vec::new();
    for i in 1..data.as_array().unwrap().len() {
        if !data[i]["statuscolor"].as_str().unwrap().to_string().is_empty() {
            all_colors.push(data[i]["statuscolor"].as_str().unwrap().to_string());
        }
    }

    // remove duplicates
    all_colors.sort();
    all_colors.dedup();

    // get the total number of results
    let total_results = all_colors.len();

    // json response
    let json_response = json!({
        "results": all_colors,
        "total_results": total_results,
    });

    Ok(warp::reply::json(&json_response))
}

async fn filter_getcategories_handler() -> Result<impl warp::Reply, warp::Rejection> {
    // fetch the data
    let data = fetch_data().await.unwrap();

    let mut all_categories: Vec<String> = Vec::new();
    for i in 1..data.as_array().unwrap().len() {
        if !data[i]["categories"].as_array().unwrap().is_empty() {
            for j in 0..data[i]["categories"].as_array().unwrap().len() {
                if !data[i]["categories"][j].as_str().unwrap().to_string().is_empty() {
                    all_categories.push(data[i]["categories"][j].as_str().unwrap().to_string());
                }
            }
        }
    }

    // remove duplicates
    all_categories.sort();
    all_categories.dedup();

    // get the total number of results
    let total_results = all_categories.len();

    // json response
    let json_response = json!({
        "results": all_categories,
        "total_results": total_results,
    });

    Ok(warp::reply::json(&json_response))
}
