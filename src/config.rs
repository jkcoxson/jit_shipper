// jkcoxson
// Represents a configuration file

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    devices: Vec<Device>,
    versions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    name: String,
    version: String,
    apps: HashMap<String, String>,
}

impl Config {
    fn new() -> Config {
        Config {
            devices: Vec::new(),
            versions: Vec::new(),
        }
    }
    pub fn load(json_path: &str) -> Config {
        match File::open(json_path) {
            Ok(mut file) => {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => match serde_json::from_str(&contents) {
                        Ok(config) => config,
                        Err(e) => {
                            println!("Error parsing config file: {}", e);
                            Config::new()
                        }
                    },
                    Err(e) => {
                        println!("Error reading config file: {}", e);
                        Config::new()
                    }
                }
            }
            Err(_) => {
                println!("Error opening config file: {}", json_path);
                Config::new()
            }
        }
    }
    pub fn save(&self) {
        let json_path = "config.json";
        match File::create(json_path) {
            Ok(mut file) => {
                let json = serde_json::to_string_pretty(&self).unwrap();
                match file.write_all(json.as_bytes()) {
                    Ok(_) => println!("Saved config file: {}", json_path),
                    Err(e) => println!("Error saving config file: {}", e),
                }
            }
            Err(e) => println!("Error opening config file: {}", e),
        }
    }
}
