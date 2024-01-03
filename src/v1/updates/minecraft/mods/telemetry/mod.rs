use std::fs::OpenOptions;
use std::io::{self, Write};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct IpInfo {
    region: String,
    // Add other fields as needed
}

fn get_ip_hash(ip: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    ip.hash(&mut hasher);
    hasher.finish()
}

pub fn log_ip_info(ip_address: &str, file_path: &str, mod_package: &str) -> io::Result<()> {
    let ip_hash = get_ip_hash(ip_address);

    let ip_info = match get_ip_info(ip_address) {
        Ok(info) => info,
        Err(err) => {
            IpInfo { region: "Unknown".to_string() } // Default to "Unknown" in case of an error
        }
    };

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "{} {} {}", ip_hash, ip_info.region, mod_package)?;

    Ok(())
}

fn main() {
    let file_path = "ip_log.txt"; // Replace with your desired file path

    // Example usage
    match log_ip_info("8.8.8.8", file_path) {
        Err(err) => eprintln!("Error: {}", err),
    }
}
