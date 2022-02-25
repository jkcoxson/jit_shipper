// jkcoxson
// All in one tool for activating JIT on iOS devices

use std::{
    env,
    fs::{self, File},
    io,
};

use config::Device;
use rusty_libimobiledevice::libimobiledevice;
mod config;
mod install;
mod user_input;
mod ui;

fn main() {
    let app = ui::JMoney::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);




    println!("#################");
    println!("## JIT Shipper ##");
    println!("##  jkcoxson   ##");
    println!("#################\n\n");

    // Get all devices attatched
    let mut devices = libimobiledevice::get_devices().unwrap();
    if devices.len() < 1 {
        println!("No devices found. Please make sure you have a device connected.\n");
        return;
    }

    println!("Found {} devices:", devices.len());
    for i in &devices {
        println!("{:?}", i);
    }

    match devices[0].new_lockdownd_client("yeet".to_string()) {
        Ok(_) => {
            println!("Lockdownd service started");
        }
        Err(e) => {
            println!("Error starting lockdown service: {:?}", e);
            return;
        }
    }
    match devices[0].start_debug_server("yeet".to_string()) {
        Ok(()) => {
            println!("Debug server started");
        }
        Err(e) => {
            println!("Error starting debug server: {:?}", e);
            return;
        }
    }
    match devices[0].start_instproxy_service("yeet".to_string()) {
        Ok(()) => {
            println!("Instproxy service started");
        }
        Err(e) => {
            println!("Error starting instproxy service: {:?}", e);
            return;
        }
    }

    todo!("The rest of this project needs to be translated to the lib");

    // Get home directory
    let home_dir = dirs::home_dir().unwrap();
    // Detect if home_dir/libimobiledevice is present
    let libimobiledevice_path = home_dir.join("libimobiledevice");
    if !libimobiledevice_path.exists() {
        // If not, create it
        fs::create_dir(libimobiledevice_path).expect("Failed to create libimobiledevice directory");
    }
    // Change directory to libimobiledevice
    env::set_current_dir(libimobiledevice_path).expect("Failed to change directory");
    ui_loop();
}

fn ui_loop() {
    loop {
        // match choose_device() {
        //     Some(device) => {
        //         let _dmg_path = get_ios_dmg(&device);
        //         let pkg_name = choose_app(&device);
        //         match device.run_app(pkg_name) {
        //             true => {
        //                 println!("Successfully launched the app");
        //             }
        //             false => {
        //                 println!("Failed to launch the app");
        //             }
        //         }
        //     }
        //     None => {
        //         println!("No devices detected, connect a device and then press enter");
        //         std::io::stdin().read_line(&mut String::new()).unwrap();
        //     }
        // }
    }
}

fn get_ios_dmg(device: &Device) -> String {
    // Get directory
    let home_dir = dirs::home_dir().unwrap();
    let libimobiledevice_path = home_dir.join("libimobiledevice");
    let ios_path = &libimobiledevice_path.join(device.version.clone());
    // Check if directory exists
    if ios_path.exists() {
        // Check if DMG exists
        let ios_dmg = ios_path.join("DeveloperDiskImage.dmg");
        if ios_dmg.exists() {
            return ios_dmg.to_str().unwrap().to_string();
        } else {
            // Remove iOS directory
            std::fs::remove_dir_all(ios_path).unwrap();
        }
    }
    // Download versions.json from GitHub
    println!("Downloading iOS dictionary...");
    let url = "https://raw.githubusercontent.com/jkcoxson/jit_shipper/master/versions.json";
    let response = reqwest::blocking::get(url).expect("Failed to download iOS version library");
    let contents = response.text().expect("Failed to read iOS version library");
    // Parse versions.json
    let versions: serde_json::Value = serde_json::from_str(&contents).unwrap();
    // Get DMG url
    let ios_dmg_url = match versions.get(device.version.as_str()) {
        Some(x) => x.as_str().unwrap().to_string(),
        None => panic!(
            "\nCould not find {} from the library. Check back later!\n",
            device.version
        ),
    };
    // Download DMG zip
    println!("Downloading iOS {} DMG...", device.version);
    let mut resp = reqwest::blocking::get(ios_dmg_url).expect("Unable to download DMG");
    let mut out = File::create("dmg.zip").expect("Failed to create zip");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
    // Unzip zip
    let mut dmg_zip = zip::ZipArchive::new(File::open("dmg.zip").unwrap()).unwrap();
    dmg_zip.extract(libimobiledevice_path).unwrap();
    // Remove zip
    std::fs::remove_file("dmg.zip").unwrap();
    // Return DMG path
    let ios_dmg = ios_path.join("DeveloperDiskImage.dmg");
    ios_dmg.to_str().unwrap().to_string()
}

fn choose_app(device: &Device) -> String {
    println!("Fetching apps installed on device...");
    let apps = device.app_scan();
    let mut options = vec![];
    for (key, _) in &apps {
        options.push(key.clone().replace("\"", ""));
    }
    options.sort();
    let options: Vec<&str> = options.iter().map(|x| x.as_str()).collect();
    let choice = user_input::multi_input("Choose an app", options.as_slice());
    return apps
        .get(format!("\"{}\"", choice).as_str())
        .unwrap()
        .clone();
}
