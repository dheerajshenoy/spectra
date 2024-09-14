use eframe::{run_native, App, NativeOptions};
use egui::{Align, CentralPanel, Layout, RichText, Ui, Widget};
use serde::de::Error;
use toml;
use std::fs;

use crate::slide::{self, Element, ElementType, Slide, SpectraFile};

// Main application state
pub struct Spectra {
    current_slide_index: usize,
    slides: Vec<Slide>,
    is_last_slide: bool,
    is_first_slide: bool,
}

// Methods for spectra
impl Spectra {

    pub fn new(sfile: SpectraFile) -> Self {
        Self {
            current_slide_index: 0,
            slides: sfile.slides,
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

    fn render_elements(&self, ui: &mut Ui, elements: &[Element]) {
        for element in elements {
            match element.oftype.as_str() {

                "text" => {
                    ui.label(RichText::new(element.content.as_deref().unwrap())
                        .size(element.font_size.unwrap_or(14.0) as f32)
                        .background_color(egui::Color32::from_hex(&element.background_color.as_deref().unwrap_or("#00000000")).unwrap())
                        .color(egui::Color32::from_hex(&element.color.as_deref().unwrap_or("#000000")).unwrap())
                    );
                }

                _ => {

                }
            }
        }
    }

    fn load_slide(&self, ui: &mut Ui, index: usize) {
        let slide = &self.slides[index];

        let layout = match slide.layout.as_str() {

            "vertical" => {
                ui.vertical(|ui| {
                    self.render_elements(ui, &slide.elements)
                });
            },

            "horizontal" => {
                ui.horizontal(|ui| {
                    self.render_elements(ui, &slide.elements)
                });
            }

            _ => {

            }
        };
        



    }



}



impl App for Spectra {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        self.is_last_slide = self.current_slide_index == self.slides.len() - 1;
        self.is_first_slide = self.current_slide_index == 0;

        CentralPanel::default().show(ctx, |ui| {

            self.load_slide(ui, self.current_slide_index);

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
