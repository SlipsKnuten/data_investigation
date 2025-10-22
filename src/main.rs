use serde::{Deserialize, Serialize};
use std::fs;
use tracing::{info, error, debug, instrument};
use tracing_subscriber;


#[derive(Debug, Deserialize, Serialize)]
struct json_data {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    gender: String,
    ip_address: String,
}

fn import_data(path: &str) -> Result<Vec<json_data>, Box<dyn std::error::Error>>{
    info!("Trying to read json from: {}", path);
    
    let contents = fs::read_to_string(path).map_err(|e| {
        error!("Failed to read file '{}': {}", path, e);
        e
    })?;

    debug!("File contents length: {} bytes", contents.len());

    let data: Vec<json_data> = serde_json::from_str(&contents).map_err(|e| {
        error!("Failed to parse JSON: {}", e);
        e
    })?;

    info!("Successfully loaded {} records", data.len());
    Ok(data)
}

fn main() {
    tracing_subscriber::fmt()
        .json()
        .init();

    info!("Starting app");


    match import_data("data/MOCK_DATA.json") {
        Ok(data) => {
            info!("Data imported");
            debug!("First record: {:?}", data.first());
            println!("Loaded {} records", data.len());
        }
        Err(e) => {
            error!("Fatal error: {}", e);
            eprintln!("Error reading JSON: {}", e);
        }
    }

}
