// jkcoxson
// Represents a configuration file

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{Read, Write},
    process::Output,
};

#[derive(Serialize, Deserialize)]
pub struct Device {
    detected: bool,
    pub name: String,
    pub version: String,
    apps: HashMap<String, String>,
    pub udid: String,
    pub network: bool,
}

impl Device {
    pub fn new(udid: String, name: String, version: String, network: bool) -> Device {
        Device {
            detected: true,
            name,
            version,
            apps: HashMap::new(),
            udid,
            network,
        }
    }

    pub fn device_scan() -> Vec<Device> {
        match env::consts::OS {
            "macos" => {
                let output = std::process::Command::new("idevice_id")
                    .output()
                    .expect("Failed to execute process");
                let output_str = String::from_utf8_lossy(&output.stdout);
                let mut devices: Vec<Device> = Vec::new();
                let first_line = match output_str.lines().nth(0) {
                    Some(x) => x,
                    None => return devices,
                };
                for line in first_line.split("\n") {
                    if line.len() < 1 {
                        continue;
                    }
                    let line = output_str.split(" ");
                    let line = line.collect::<Vec<&str>>();
                    let udid = line[0].trim();
                    let network = match line[1].trim() {
                        "(Network)" => true,
                        _ => false,
                    };
                    let mut found = false;
                    for device in devices.iter_mut() {
                        if device.udid == udid {
                            found = true;
                        }
                        if found && network {
                            device.network = true;
                        }
                    }
                    if found {
                        continue;
                    }

                    let info: Output;
                    // Get device info
                    if network {
                        info = std::process::Command::new("ideviceinfo")
                            .arg("-u")
                            .arg("-n")
                            .arg(udid)
                            .output()
                            .expect("Failed to execute process");
                    } else {
                        info = std::process::Command::new("ideviceinfo")
                            .arg("-u")
                            .arg(udid)
                            .output()
                            .expect("Failed to execute process");
                    }

                    let info = String::from_utf8_lossy(&info.stdout);
                    let mut name = "";
                    let mut verson = "";
                    for line in info.split("\n") {
                        if line.len() < 1 {
                            continue;
                        }
                        let line = line.split(":");
                        let line = line.collect::<Vec<&str>>();
                        if line[0] == "DeviceName" {
                            name = line[1].trim();
                        }
                        if line[0] == "ProductVersion" {
                            verson = line[1].trim();
                        }
                    }

                    let device = Device::new(
                        udid.to_string(),
                        name.to_string(),
                        verson.to_string(),
                        network,
                    );
                    devices.push(device);
                }
                devices
            }
            // "linux" => {}
            // "windows" => {}
            _ => {
                panic!("Unsupported OS");
            }
        }
    }
}
