use eframe::{run_native, App, NativeOptions};
use egui::{Align, CentralPanel, Layout};
use slide::SpectraFile;
use spectra::Spectra;
use toml;
use std::fs;

mod slide;
mod spectra;

fn load_presentation_from_toml(file_path: &str) -> SpectraFile {
    let toml_str = fs::read_to_string(file_path).expect("Failed to read TOML file");
    toml::from_str(&toml_str).expect("Failed to parse TOML")
}

fn main() -> eframe::Result {
    let app : Spectra = Spectra::new(&load_presentation_from_toml("/home/neo/Gits/spectra/src/present.toml"));
    let win_opts = NativeOptions::default();
    run_native("Spectra",
        win_opts,
        Box::new(|_cc| {
            Ok(Box::new(app))
        }),
    )
}