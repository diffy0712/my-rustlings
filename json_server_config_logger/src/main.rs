use serde::{Serialize, Deserialize};
use std::{net::{IpAddr}, fmt::Display};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
enum ServerStatus {
    Up,
    Down
}

impl Display for ServerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerStatus::Up => write!(f, "Up"),
            ServerStatus::Down => write!(f, "Down"),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    host: String,
    ip: IpAddr,
    status: ServerStatus
}

fn main() { 
    println!("Loading config file");

    let config_contents = fs::read_to_string("./configs/default.json".to_string())
        .expect("Should have been able to read the file");

    // Convert the JSON string back to a Point.
    let server_config: ServerConfig = serde_json::from_str(&config_contents).unwrap();

    println!("Contents of config file:");
    println!("  Host: {}", server_config.host);
    println!("  Ip: {}", server_config.ip);
    println!("  Status: {}", server_config.status);
}