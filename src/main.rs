use eframe::{run_native, App, NativeOptions};
use egui::{Align, CentralPanel, Layout, Response, Ui};
use toml;

// Main application state
struct Spectra {
    nslides: u32,
    current_slide_index: u32
}

// Methods for spectra
impl Spectra {

    // Goto prev slide
    fn prev_slide(&mut self) {

        if self.current_slide_index > 0 {
            self.current_slide_index -= 1;
        }
    }

    // Goto next slide
    fn next_slide(&mut self) {
        if self.current_slide_index < self.nslides {
            self.current_slide_index += 1;
        }
    }
}

impl Default for Spectra {
    fn default() -> Self {
        Self {
            nslides: 10,
            current_slide_index: 0,
        }
    }
}

impl App for Spectra {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        let is_last_slide : bool = self.current_slide_index == self.nslides;
        let is_first_slide : bool = self.current_slide_index == 0;

        CentralPanel::default().show(ctx, |ui| {

            // Prev and Next Slide Buttons
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {

                // Previous Slide Button
                ui.add_enabled_ui(!is_first_slide, |ui| {
                    let prev_btn = ui.button("Previous Slide");
                    prev_btn.clicked().then( || {
                        self.prev_slide();
                    });
                });

                // Next Slide Button
                ui.add_enabled_ui(!is_last_slide, |ui| {
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
    }

}

fn main() -> eframe::Result {
    let win_opts = NativeOptions::default();
    run_native("Spectra",
        win_opts,
        Box::new(|_cc| {
            Ok(Box::<Spectra>::default())
        }),
    )
}