// jkcoxson
// All in one tool for activating JIT on iOS devices

mod backend;
mod tui;
mod ui;

fn main() {
    println!("#################");
    println!("## JIT Shipper ##");
    println!("##  jkcoxson   ##");
    println!("#################\n\n");
    println!("If you have issues starting the GUI, run with --fallback to run the TUI instead.\n");

    let args = std::env::args().collect::<Vec<_>>();
    if args.contains(&"--fallback".to_string()) {
        tui::run();
    } else {
        let app = ui::JMoney::default();
        let native_options = eframe::NativeOptions::default();
        eframe::run_native(Box::new(app), native_options);
    }
    
}

