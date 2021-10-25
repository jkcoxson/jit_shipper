// jkcoxson
// Represents a configuration file

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, process::Output};

#[derive(Serialize, Deserialize)]
pub struct Device {
    detected: bool,
    pub name: String,
    pub version: String,
    pub udid: String,
}

impl Device {
    pub fn new(udid: String, name: String, version: String) -> Device {
        Device {
            detected: true,
            name,
            version,
            udid,
        }
    }

    pub fn device_scan() -> Vec<Device> {
        match env::consts::OS {
            "macos" => {
                let output = std::process::Command::new("idevice_id")
                    .arg("-l")
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
                    let udid = output_str.trim();
                    let mut found = false;
                    for device in devices.iter_mut() {
                        if device.udid == udid {
                            found = true;
                        }
                    }
                    if found {
                        continue;
                    }

                    let info: Output;
                    // Get device info
                    info = std::process::Command::new("ideviceinfo")
                        .arg("-u")
                        .arg(udid)
                        .output()
                        .expect("Failed to execute process");

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

                    let device =
                        Device::new(udid.to_string(), name.to_string(), verson.to_string());
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

    pub fn app_scan(&self) -> HashMap<String, String> {
        let lines = self.return_idi();
        let lines = lines.split("\n");
        let mut apps: HashMap<String, String> = HashMap::new();
        let mut flush = false;
        for line in lines {
            if !flush {
                flush = true;
                continue;
            }
            if line.len() < 1 {
                continue;
            }
            let line = line.split(",");
            let line = line.collect::<Vec<&str>>();
            let identifier = line[0].trim();
            let name = line[2].trim();
            apps.insert(name.to_string(), identifier.to_string());
        }
        println!("{:?}", apps);
        apps
    }

    pub fn return_idi(&self) -> String {
        match env::consts::OS {
            "macos" => {
                let output = std::process::Command::new("ideviceinstaller")
                    .arg("-l")
                    .arg("-u")
                    .arg(self.udid.clone())
                    .output()
                    .unwrap();
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            _ => {
                panic!("Unsupported OS");
            }
        }
    }

    pub fn run_app(&self, pkg_identifier: String) -> bool {
        match env::consts::OS {
            "macos" => {
                let output = std::process::Command::new("idevicedebug")
                    .arg("-u")
                    .arg(self.udid.clone())
                    .arg("--detach")
                    .arg("run")
                    .arg(pkg_identifier)
                    .output()
                    .expect("Failed to execute process");
                let error = String::from_utf8_lossy(&output.stderr);
                if error.len() > 0 {
                    println!("{}", error);
                    return false;
                }
                true
            }
            _ => {
                panic!("Unsupported OS");
            }
        }
    }
}
