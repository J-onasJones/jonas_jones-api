use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::logger::Logger;
use std::env;

pub fn parse_ip() -> SocketAddr {

    let env_api_ip = return_env_var("API_IP");

    let env_api_port = return_env_var("API_PORT");

    let result: Result<Vec<i32>, _> = env_api_ip
        .split('.')
        .map(|s| s.parse::<i32>())
        .collect();

    let ip_parts = match result {
        Ok(numbers) => numbers,
        Err(_4) => {
            Logger::panic(&format!("Illegal character in Environment variable 'API_IP'"));
            std::process::exit(1)
        }
    };

    // Attempt to parse the string into a u16
    let parsed_env_api_port: u16 = match env_api_port.parse::<u16>() {
        Ok(parsed_env_api_port) => parsed_env_api_port,
        Err(_) => {
            Logger::panic(&format!("Illegal character in Environment variable 'Api_PORT'"));
            std::process::exit(1)
        }
    };

    // Convert Vec<i32> to Ipv4Addr
    let ipv4_addr: Ipv4Addr = Ipv4Addr::new(
        ip_parts[0] as u8,
        ip_parts[1] as u8,
        ip_parts[2] as u8,
        ip_parts[3] as u8,
    );

    // Convert Ipv4Addr to IpAddr
    let ip_addr: IpAddr = IpAddr::V4(ipv4_addr);

    // Create SocketAddr from IpAddr and port
    let socket_addr: SocketAddr = SocketAddr::new(ip_addr, parsed_env_api_port);

    return socket_addr;
}



pub fn return_env_var(key: &str) -> String {
    return match env::var(key) {
        Ok(value) => value,
        Err(_) => {
            Logger::panic(&format!("Environment variable '{}' not found", key));
            std::process::exit(1);
        }
    };
}