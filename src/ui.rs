// jkcoxson

use eframe::{egui::{self, RichText, FontId}, epi};
use rusty_libimobiledevice::{libimobiledevice::{Device, self}, lockdownd::LockdowndClient};

pub struct JMoney { // Maybe a good *wrapper* name?
    chosen_device: Option<u8>,
    device_list: Option<Vec<(Device, LockdowndClient, String)>>,
    dmg_path: Option<String>,

    // Show specific windows
    show_about: bool,
    error: Option<String>,
}

impl Default for JMoney {
    fn default() -> Self {
        Self {
            chosen_device: None,
            device_list: None,
            dmg_path: None,

            // Show specific windows
            show_about: false,
            error: None,
        }
    }
}

impl epi::App for JMoney {
    fn name(&self) -> &str {
        "JIT Shipper"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self { chosen_device, device_list, dmg_path, show_about, error } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("About").clicked() {
                        *show_about = true;
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });


        egui::CentralPanel::default().show(ctx, |ui| {
            // Fetch device list
            if device_list.is_none() {
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
                *device_list = Some(to_return);
            }
            ui.heading("Choose a device");
            egui::ComboBox::from_label("").selected_text(format!("{}", match chosen_device {
                Some(i) => device_list.as_ref().unwrap()[*i as usize].2.clone(),
                None => "None".to_string(),
            })).show_ui(ui, |ui| {
                let mut i = 0;
                for device in (*device_list).as_ref().unwrap().into_iter() {
                    if ui.button(String::from(device.2.clone())).clicked() {
                        *chosen_device = Some(i);
                        *dmg_path = None;
                    }
                    i += 1;
                }
            });
            if ui.button("Refresh").clicked() {
                *device_list = None;
                *chosen_device = None;
                *dmg_path = None;
            }
            if chosen_device.is_some() {
                if dmg_path.is_none() {
                    let ios_version = (*device_list).as_ref().unwrap()[chosen_device.unwrap() as usize].1.get_value("ProductVersion".to_string(), "".to_string()).unwrap().get_string_val().unwrap();
                    println!("iOS version: {}", ios_version);
                    *dmg_path = match crate::get_ios_dmg(ios_version) {
                        Ok(dmg_path) => Some(dmg_path),
                        Err(e) => {
                            *error = Some(e);
                            *chosen_device = None;
                            None
                        }
                    };
                }
            }
            egui::warn_if_debug_build(ui);
        });


        // Windows
        if *show_about {
            egui::Window::new("About").show(ctx, |ui| {
                ui.label(RichText::new("JIT Shipper").font(FontId::proportional(20.0)).underline());
                ui.label("Written by your boi jkcoxson");
                ui.label("v0.1.0");
                ui.label("All hail camels o7");
                if ui.button("Close").clicked() {
                    *show_about = false;
                }
            });
        }
        if (*error).is_some() {
            egui::Window::new("Error").show(ctx, |ui| {
                ui.label((*error).as_ref().unwrap());
                if ui.button("Close").clicked() {
                    *error = None;
                }
            });
        }
    }
}