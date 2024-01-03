use std::collections::HashMap;
use std::net::IpAddr;
use reqwest::Error;
use serde_json::json;
use serde::{Deserialize, Serialize};
use warp::Filter;
use regex::Regex;

use crate::error_responses::BadRequestError;

pub fn get_mods_paths() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // any path that starts with /v1/updates/minecraft/mods/{modname}/{loadername}/{version} calls handle_path
    warp::path("v1").and(warp::path("updates")).and(warp::path("minecraft")).and(warp::path("mods"))

        .and(warp::get().and(warp::path::param()).and(warp::path::param()).and(warp::path::param()).and(warp::path::param()).and(warp::path::end()).and(warp::filters::header::headers_cloned()).and(warp::query::<HashMap<String, String>>()).and_then(handle_path))
}

#[derive(Debug, Deserialize, Serialize)]
struct ModData {
    package: String,
    name: String,
    versions: Vec<HashMap<String, HashMap<String, ModVersion>>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ModVersion {
    recommended: String,
    latest: String,
    all: Vec<String>,
}

// get json data from https://https://cdn.jonasjones.dev/api/mcmods/mcmod_metadata.json
pub async fn fetch_data() -> Result<serde_json::Value, Error> {
    let url = "https://cdn.jonasjones.dev/api/mcmods/mcmod_metadata.json";
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

fn is_valid_ip(ip_str: &str) -> bool {
    if let Ok(ip) = ip_str.parse::<IpAddr>() {
        match ip {
            IpAddr::V4(_) => true,
            IpAddr::V6(_) => true,
        }
    } else {
        false
    }
}

fn is_valid_minecraft_version(version: &str) -> bool {
    // Define the regex pattern for the Minecraft version
    let pattern = Regex::new(r"^1\.\d{1,2}(\.\d)?$").unwrap();

    // Check if the provided version matches the pattern
    pattern.is_match(version)
}

fn get_header_forward_for_ip(headers: warp::http::HeaderMap) -> String {
    // check if the header X-Forward-For exists and return the ip, if not, return an empty string
    if let Some(forwarded_for) = headers.get("X-Forwarded-For") {
        if let Ok(ip) = forwarded_for.to_str() {
            // Extract the first IP address from the comma-separated list
            if let Some(first_ip) = ip.split(',').next() {
                return first_ip.trim().to_string();
            }
        }
    }
    String::new()
}

async fn handle_path(modpackage: String, loadername: String, mcversion: String, modversion: String, headers: warp::http::HeaderMap, params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // Retrieve the IP from the header and check if it's valid
    let mut client_ip = get_header_forward_for_ip(headers);
    if !is_valid_ip(&client_ip) {
        client_ip = params.get("ip").unwrap_or(&"".to_string()).to_string();
        if !is_valid_ip(&client_ip) {
            client_ip = "Not valid".to_string();
        }
    }

    // check if the minecraft version is valid
    if !is_valid_minecraft_version(&mcversion) {
        return Err(warp::reject::custom(BadRequestError));
    }

    // fetch the data
    let data = fetch_data().await.unwrap();

    // filter the data
    // convert the raw list of data into a list of ModData and ModVersion
    let mods_data: Vec<ModData> = serde_json::from_value(data).unwrap();

    // get the mod data from the requested mod
    let mod_data: ModData = mods_data.into_iter().find(|mod_data| mod_data.package == modpackage).unwrap();


    // get the version data from the requested loader and remove the other loaders
    let version_data: HashMap<std::string::String, ModVersion> = mod_data.versions.into_iter().find(|version_data| version_data.contains_key(&loadername)).unwrap().remove(&loadername).unwrap();

    // turn version_data into an object of String: ModVersion key value pairs
    let version_data: HashMap<std::string::String, ModVersion> = version_data.into_iter().map(|(key, value)| (key, value)).collect();

    // get the version data for the current minecraft version
    let version_data: ModVersion = version_data.get(&mcversion).unwrap().clone();

    // get recommended and latest version
    let recommended_version = version_data.recommended.clone();
    let latest_version = version_data.latest.clone();

    // determine whether the client is up to date
    let mut up_to_date = false;
    if modversion == recommended_version {
        up_to_date = true;
    }

    // determine if telemetry is enabled by checking if the client_ip is valid
    let mut telemetry = false;
    if is_valid_ip(&client_ip) {
        telemetry = true;
    }

    // create the response
    let response = json!({
        "promos": {
            "latest": latest_version,
            "recommended": recommended_version
        },
        "upToDate": up_to_date,
        "telemetry_enabled": telemetry
    });

    //TODO: Add way to process telemetry data

    // return the data
    return Ok(warp::reply::json(&response));
}
