// jkcoxson
// All in one tool for activating JIT on iOS devices

use std::{
    fs::{ File},
    io, path::PathBuf,
};

mod ui;

fn main() {
    println!("#################");
    println!("## JIT Shipper ##");
    println!("##  jkcoxson   ##");
    println!("#################\n\n");

    let app = ui::JMoney::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

pub fn get_ios_dmg(version: String) -> Result<String, String> {
    // Get directory
    let home_dir = dirs::home_dir().unwrap();
    let libimobiledevice_path = home_dir.join("JIT Shipper");

    // Check if directory exists
    if libimobiledevice_path.join(format!("{}.dmg", &version)).exists() {
        return Ok(libimobiledevice_path.join(format!("{}.dmg", &version)).to_str().unwrap().to_string());
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
    Ok(libimobiledevice_path.join(format!("{}.dmg", &version)).to_str().unwrap().to_string())
}
