
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Logger;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    host: String,
    port: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LastFMConfig {
    api_key: String,
    api_secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    host: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    server: ServerConfig,
    lastfm: LastFMConfig,
    database: DatabaseConfig,
}

pub fn load_config() -> (ServerConfig, LastFMConfig, DatabaseConfig) {
    // load config.toml. create new one if it doesn't exist
    // return config
    let config_path = "config.toml";
    if !std::path::Path::new(config_path).exists() {
        // If it doesn't exist, create a new config file with default values
        create_default_config(config_path);
    }

    // Read the configuration file
    let mut config_file = File::open("config.toml").expect({Logger::panic("Failed to open config file"); std::process::exit(1)});
    let mut config_toml = String::new();
    config_file
        .read_to_string(&mut config_toml)
        .expect("Failed to read config file");

    // Deserialize the TOML into the AppConfig struct
    let config: Config = toml::from_str(&config_toml).expect({Logger::panic("Failed to deserialize config file"); std::process::exit(1)});

    // Return the config
    return (config.server, config.lastfm, config.database);
}

fn create_default_config(path: &str) {
    // Create default Config
    let default_config = Config {
        server: ServerConfig {
            host: String::from("localhost"),
            port: 8080,
        },
        lastfm: LastFMConfig {
            api_key: String::from(""),
            api_secret: String::from(""),
        },
        database: DatabaseConfig {
            host: String::from(""),
        },
    };

    // Serialize default config to TOML
    let toml_string = toml::to_string_pretty(&default_config).expect("Failed to serialize config");

    // Write the TOML string to the config file
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Failed to open config file for writing");

    file.write_all(toml_string.as_bytes())
        .expect("Failed to write to config file");
}