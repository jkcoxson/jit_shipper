// jkcoxson

use dialoguer::{Select, theme::ColorfulTheme};

use crate::backend::Backend;

pub fn run() {
    let mut backend = Backend::new();

    backend.get_device_list();

    let choices = backend.device_list.as_ref().unwrap().iter().map(|(device, _, name)| {
        format!("{} - {}", device.udid, name)
    }).collect::<Vec<String>>();

    if choices.len() == 0 {
        println!("No devices found");
        return;
    }
    
    // Get a device choice
    let device_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a device")
        .items(&choices)
        .default(0)
        .interact()
        .unwrap();

    backend.chosen_device = Some(device_choice as u8);
    let version = backend.device_list.as_ref().unwrap()[backend.chosen_device.unwrap() as usize].1.get_value("ProductVersion".to_string(), "".to_string()).unwrap().get_string_val().unwrap();
    match backend.get_ios_dmg(version) {
        Ok(_) => {
            println!("Successfully loaded DMG");
        }
        Err(e) => {
            println!("{}", e);
            return;
        }
    }
}

pub fn multi_input(prompt: &str, options: &[&str]) -> String {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();
    return options[selection].to_string();
}
