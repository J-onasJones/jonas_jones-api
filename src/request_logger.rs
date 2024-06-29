use chrono::Utc;
use reqwest::Client;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use warp::http::Method;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct RequestLog {
    timestamp: String,
    method: String,
    pathname: String,
    ip_country_code: String,
    ip_hash: String,
}

async fn get_ip_country_code(ip: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("http://ip-api.com/json/{}", ip);
    let response: Value = get(&url)?.json()?;
    if let Some(country_code) = response["countryCode"].as_str() {
        Ok(country_code.to_string())
    } else {
        Err("Could not fetch country code".into())
    }
}

async fn hash_ip(ip: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(ip);
    format!("{:x}", hasher.finalize())
}

pub async fn log_request(client: &Client, ip: &str, pathname: &str, method: &Method, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let method_str = method.as_str();
    let timestamp = Utc::now().to_rfc3339();
    let ip_country_code = get_ip_country_code(ip).await?;
    let ip_hash = hash_ip(ip).await;

    let log_entry = RequestLog {
        timestamp,
        method: method_str.to_string(),
        pathname: pathname.to_string(),
        ip_country_code,
        ip_hash,
    };

    let path = Path::new(file_path);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let mut logs: Vec<RequestLog> = if path.exists() {
        let reader = BufReader::new(&file);
        serde_json::from_reader(reader)?
    } else {
        Vec::new()
    };

    logs.push(log_entry);

    let writer = BufWriter::new(&file);
    serde_json::to_writer_pretty(writer, &logs)?;

    Ok(())
}