// jkcoxson

use eframe::{egui::{self, RichText, FontId}, epi};

use crate::backend::Backend;

pub struct JMoney { // Maybe a good *wrapper* name?
    backend: Backend,
}

impl Default for JMoney {
    fn default() -> Self {
        Self {
            backend: Backend::new(),
        }
    }
}

impl epi::App for JMoney {
    fn name(&self) -> &str {
        "JIT Shipper"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self { backend } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("About").clicked() {
                        backend.show_about = true;
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Fetch device list
            if backend.device_list.is_none() {
                backend.get_device_list();
            }
            ui.heading("Choose a device");
            egui::ComboBox::from_label("").selected_text(format!("{}", match backend.chosen_device {
                Some(i) => backend.device_list.as_ref().unwrap()[i as usize].2.clone(),
                None => "None".to_string(),
            })).show_ui(ui, |ui| {
                let mut i = 0;
                for device in (backend.device_list).as_ref().unwrap().into_iter() {
                    if ui.button(String::from(device.2.clone())).clicked() {
                        backend.chosen_device = Some(i);
                        backend.dmg_path = None;
                    }
                    i += 1;
                }
            });
            if ui.button("Refresh").clicked() {
                backend.device_list = None;
                backend.chosen_device = None;
                backend.dmg_path = None;
            }
            if backend.chosen_device.is_some() {
                if backend.dmg_path.is_none() {
                    let ios_version = (backend.device_list).as_ref().unwrap()[backend.chosen_device.unwrap() as usize].1.get_value("ProductVersion".to_string(), "".to_string()).unwrap().get_string_val().unwrap();
                    println!("iOS version: {}", ios_version);
                    match backend.get_ios_dmg(ios_version) {
                        Ok(_) => {},
                        Err(e) => {
                            backend.error = Some(e);
                            backend.chosen_device = None;
                        }
                    };
                }
            }
            egui::warn_if_debug_build(ui);
        });


        // Windows
        if backend.show_about {
            egui::Window::new("About").show(ctx, |ui| {
                ui.label(RichText::new("JIT Shipper").font(FontId::proportional(20.0)).underline());
                ui.label("Written by your boi jkcoxson");
                ui.label("v0.1.0");
                ui.label("All hail camels o7");
                if ui.button("Close").clicked() {
                    backend.show_about = false;
                }
            });
        }
        if (backend.error).is_some() {
            egui::Window::new("Error").show(ctx, |ui| {
                ui.label((backend.error).as_ref().unwrap());
                if ui.button("Close").clicked() {
                    backend.error = None;
                }
            });
        }
    }
}