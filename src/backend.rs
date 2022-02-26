// jkcoxson

use std::{fs::File, io, path::PathBuf};

use rusty_libimobiledevice::{libimobiledevice::{Device, self}, lockdownd::LockdowndClient};

pub struct Backend {
    pub chosen_device: Option<u8>,
    pub device_list: Option<Vec<(Device, LockdowndClient, String)>>,
    pub dmg_path: Option<String>,

    // Show specific windows on the GUI
    pub show_about: bool,
    pub error: Option<String>,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            chosen_device: None,
            device_list: None,
            dmg_path: None,

            // Show specific windows
            show_about: false,
            error: None,
        }
    }

    pub fn get_device_list(&mut self) {
        println!("Fetching device list...");
        let mut to_return = vec![];
        let devices = libimobiledevice::get_devices().unwrap();
        for mut i in devices {
            let ldc = match i.new_lockdownd_client("jitshipper".to_string()) {
                Ok(ldc) => ldc,
                Err(e) => {
                    println!("Error starting lockdownd service for {}: {:?}", i.udid, e);
                    continue;
                }
            };
            let name = match ldc.get_value("DeviceName".to_string(), "".to_string()) {
                Ok(name) => name,
                Err(e) => {
                    println!("Error getting device name for {}: {:?}", i.udid, e);
                    continue;
                }
            };
            to_return.push((i, ldc, name.get_string_val().unwrap()));
        }
        self.device_list = Some(to_return);
    }

    pub fn get_ios_dmg(&mut self, version: String) -> Result<(), String> {
        // Get directory
        let home_dir = dirs::home_dir().unwrap();
        let libimobiledevice_path = home_dir.join("JIT Shipper");

        // Check if directory exists
        if libimobiledevice_path.join(format!("{}.dmg", &version)).exists() {
            self.dmg_path = Some(libimobiledevice_path.join(format!("{}.dmg", &version)).to_str().unwrap().to_string());
            return Ok(());
        }
        // Download versions.json from GitHub
        println!("Downloading iOS dictionary...");
        let url = "https://raw.githubusercontent.com/jkcoxson/jit_shipper/master/versions.json";
        let response = match reqwest::blocking::get(url) {
            Ok(response) => response,
            Err(_) => {
                return Err("Error downloading versions.json".to_string());
            }
        };
        let contents = match response.text() {
            Ok(contents) => contents,
            Err(_) => {
                return Err("Error reading versions.json".to_string());
            }
        };
        // Parse versions.json
        let versions: serde_json::Value = serde_json::from_str(&contents).unwrap();
        // Get DMG url
        let ios_dmg_url = match versions.get(version.clone()) {
            Some(x) => x.as_str().unwrap().to_string(),
            None => return Err("DMG library does not contain your iOS version".to_string()),
        };
        // Download DMG zip
        println!("Downloading iOS {} DMG...", version.clone());
        let mut resp = match reqwest::blocking::get(ios_dmg_url) {
            Ok(resp) => resp,
            Err(_) => {
                return Err("Error downloading DMG".to_string());
            }
        };
        let mut out = match File::create("dmg.zip") {
            Ok(out) => out,
            Err(_) => {
                return Err("Error creating temp DMG.zip".to_string());
            }
        };
        match io::copy(&mut resp, &mut out) {
            Ok(_) => (),
            Err(_) => {
                return Err("Error writing temp DMG".to_string());
            }
        };
        // Create tmp path
        let tmp_path = libimobiledevice_path.join("tmp");
        std::fs::create_dir_all(&tmp_path).unwrap();
        // Unzip zip
        let mut dmg_zip = zip::ZipArchive::new(File::open("dmg.zip").unwrap()).unwrap();
        match dmg_zip.extract(&tmp_path) {
            Ok(_) => {}
            Err(e) => return Err(format!("Failed to unzip DMG: {:?}", e)),
        }
        // Remove zip
        match std::fs::remove_file("dmg.zip") {
            Ok(_) => (),
            Err(_) => return Err("Failed to remove DMG.zip".to_string()),
        }
        // Get folder name in tmp
        let mut dmg_path = PathBuf::new();
        for entry in std::fs::read_dir(&tmp_path).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                dmg_path = entry.path();
            }
        }
        // Move DMG to JIT Shipper directory
        let ios_dmg = dmg_path.join("DeveloperDiskImage.dmg");
        std::fs::rename(ios_dmg, libimobiledevice_path.join(format!("{}.dmg", &version))).unwrap();
        let ios_sig = dmg_path.join("DeveloperDiskImage.dmg.signature");
        std::fs::rename(ios_sig, libimobiledevice_path.join(format!("{}.dmg.signature", &version))).unwrap();

        // Remove tmp path
        std::fs::remove_dir_all(tmp_path).unwrap();
        println!("Successfully downloaded and extracted iOS {} developer disk image", version);

        // Return DMG path
        self.dmg_path = Some(libimobiledevice_path.join(format!("{}.dmg", &version)).to_str().unwrap().to_string());
        return Ok(());
    }
}