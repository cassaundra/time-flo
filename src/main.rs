#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use eframe::egui::vec2;

fn main() {
    let app = time_flo::TimeFloApp::default();
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(vec2(225., 150.)),
        resizable: false,
        transparent: true,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}
