use eframe::{run_native, App, NativeOptions};
use egui::{Align, CentralPanel, Layout};
use serde::de::Error;
use toml;
use std::fs;

use crate::slide::{self, SpectraFile};

// Main application state
pub struct Spectra {
    nslides: u32,
    current_slide_index: u32,

    is_last_slide: bool,
    is_first_slide: bool,
}

// Methods for spectra
impl Spectra {

    pub fn new(sfile: &SpectraFile) -> Self {

        Self {
            nslides: sfile.slides.len() as u32,
            current_slide_index: 0,
            is_first_slide: true,
            is_last_slide: false
        }
    }

    // Goto prev slide
    fn prev_slide(&mut self) {
        if !self.is_first_slide {
            self.current_slide_index -= 1;
        }
    }

    // Goto next slide
    fn next_slide(&mut self) {
        if !self.is_last_slide {
            self.current_slide_index += 1;
        }
    }
}

impl App for Spectra {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        self.is_last_slide = self.current_slide_index == self.nslides - 1;
        self.is_first_slide = self.current_slide_index == 0;

        CentralPanel::default().show(ctx, |ui| {

            // Navigation Buttons
            ui.with_layout(Layout::left_to_right(Align::BOTTOM), |ui| {
                // Previous Slide Button
                ui.add_enabled_ui(!self.is_first_slide, |ui| {
                    let prev_btn = ui.button("Previous Slide");
                    prev_btn.clicked().then( || {
                        self.prev_slide();
                    });
                });

                // Next Slide Button
                ui.add_enabled_ui(!self.is_last_slide, |ui| {
                    let next_btn = ui.button("Next Slide");
                    next_btn.clicked().then( || {
                        self.next_slide();
                    });
                });

                // Current Slide Index Text
                ui.label("Current Slide: ".to_string() +
                    &(self.current_slide_index + 1).to_string());

            });
        });


        // Handle keyboard input for slide navigation
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            if !self.is_last_slide {
                self.next_slide();
            }
        }

        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            if !self.is_first_slide {
                self.prev_slide();
            }
        }
    }

}
