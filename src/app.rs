use eframe::{egui::{self, ScrollArea}, epi};
use super::file::*;
pub struct CodeShare {
    text_buf: String,
    is_open_window: bool,
    file_to_open: String,
}

impl Default for CodeShare {
    fn default() -> Self {
        Self {
            text_buf: "Hello World!".to_owned(),
            is_open_window: false,
            file_to_open: String::new(),
        }
    }
}

impl epi::App for CodeShare {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { text_buf, is_open_window,  file_to_open, } = self;


        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Open").clicked(){
                        *is_open_window = true;
                    }
                    if ui.button("Save").clicked() {
                        unimplemented!()
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });
        //  Open file window
        if *is_open_window {
            egui::Window::new("Open File").show(ctx, |ui| {
                ui.text_edit_singleline(file_to_open);
                ui.horizontal( |ui| {
                    if ui.button("Open").clicked() {
                        *text_buf = open_file(file_to_open);
                        *is_open_window = false;
                    }
                    if ui.button("Close").clicked() {
                        *is_open_window = false;
                    }   
                });
            });
        }

        egui::CentralPanel::default().frame(egui::Frame::none().corner_radius(0.0)).show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui|{
                ui.add_sized(ui.available_size(),
                    egui::TextEdit::multiline(text_buf)
                        .text_style(egui::TextStyle::Monospace)
                        .code_editor()
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                )
            });
        });

    }
}
