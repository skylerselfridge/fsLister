#![windows_subsystem = "windows"]

mod app;
mod color;
mod config;
mod defines;
mod fsLister;
mod resources;
mod serde_helpers;

use config::AppConfig;
use eframe::egui::vec2;
use winapi::um::wincon::FreeConsole;

fn main() {
    unsafe {
       FreeConsole();
    }
    let config = AppConfig::load().unwrap_or_default();
    let native_options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(vec2(450.0, 520.0)),
        decorated: false,
        always_on_top: config.always_on_top,
        ..Default::default()
    };

    eframe::run_native(
        crate::defines::APP_NAME,
        native_options,
        Box::new(move |cc| Box::new(app::App::from_config(config, cc))),
    );
}
