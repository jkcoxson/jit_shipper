// jkcoxson
// Represents a configuration file

use std::{collections::HashMap, env, process::Output};

pub struct Device {
    pub name: String,
    pub version: String,
    pub udid: String,
}

impl Device {
    pub fn new(udid: String, name: String, version: String) -> Device {
        Device {
            name,
            version,
            udid,
        }
    }

    pub fn device_scan() -> Vec<Device> {
        let output = Device::idevice_id();
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
            info = Device::ideviceinfo(udid.to_string());

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

            let device = Device::new(udid.to_string(), name.to_string(), verson.to_string());
            devices.push(device);
        }
        devices
    }

    /// Runs the idevice_id command and returns the output
    pub fn idevice_id() -> Output {
        match env::consts::OS {
            "macos" => std::process::Command::new("idevice_id")
                .arg("-l")
                .output()
                .expect("Failed to execute process"),
            "linux" => std::process::Command::new("idevice_id")
                .arg("-l")
                .output()
                .expect("Failed to execute process"),
            "windows" => std::process::Command::new("powershell")
                .arg("libimobiledevice/idevice_id.exe")
                .arg("-l")
                .output()
                .expect("Failed to execute process"),
            _ => panic!("Unsupported OS"),
        }
    }

    /// Runs the ideviceinfo command and returns the output
    pub fn ideviceinfo(uuid: String) -> Output {
        match env::consts::OS {
            "macos" => std::process::Command::new("ideviceinfo")
                .arg("-u")
                .arg(uuid)
                .output()
                .expect("Failed to execute process"),
            "linux" => std::process::Command::new("ideviceinfo")
                .arg("-u")
                .arg(uuid)
                .output()
                .expect("Failed to execute process"),
            "windows" => std::process::Command::new("powershell")
                .arg("libimobiledevice/ideviceinfo.exe")
                .arg("-u")
                .arg(uuid)
                .output()
                .expect("Failed to execute process"),
            _ => panic!("Unsupported OS"),
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
            "linux" => {
                let output = std::process::Command::new("ideviceinstaller")
                    .arg("-l")
                    .arg("-u")
                    .arg(self.udid.clone())
                    .output()
                    .unwrap();
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            "windows" => {
                let output = std::process::Command::new("powershell")
                    .arg("libimobiledevice/ideviceinstaller.exe")
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
            "linux" => {
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
            "windows" => {
                let output = std::process::Command::new("powershell")
                    .arg("libimobiledevice/idevicedebug.exe")
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
