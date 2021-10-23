// jkcoxson
// All in one tool for activating JIT on iOS devices

use std::env;
mod config;
mod install;
mod user_input;

fn main() {
    println!("#################");
    println!("## JIT Shipper ##");
    println!("##  jkcoxson   ##");
    println!("#################\n\n");

    // Get home directory
    let home_dir = dirs::home_dir().unwrap();
    // Detect if home_dir/libimobiledevice is present
    let libimobiledevice_path = home_dir.join("libimobiledevice");
    if libimobiledevice_path.exists() {
        println!("libimobiledevice is installed");
        ui_loop();
    } else {
        println!("libimobiledevice is NOT installed");
        if user_input::yes_no_prompt("Would you like to install libimobiledevice?") {
            install::install();
        } else {
            println!("Exiting...");
            return;
        }
        ui_loop();
    }
}

fn ui_loop() {
    // Get home directory
    let home_dir = dirs::home_dir().unwrap();
    // Detect if home_dir/libimobiledevice is present
    let libimobiledevice_path = home_dir.join("libimobiledevice");
    let config_path = &libimobiledevice_path.join("config.json");
    if !config_path.exists() {
        std::fs::write(config_path.clone(), "{}").unwrap();
    }
    let config = config::Config::load(&config_path.to_str().unwrap());
    loop {
        env::set_current_dir(&libimobiledevice_path).unwrap();
        match user_input::multi_input(
            "What would you like to do?",
            &["Launch an application", "Add an app", "Quit"],
        )
        .as_str()
        {
            "Launch an application" => {}
            "Add an app" => {}
            "Quit" => {
                break;
            }
            _ => {
                panic!("Unhandled option");
            }
        }
    }
}
