use eframe::{egui::{self, ScrollArea}, epi};
use super::file::*;
pub struct CodeShare {
    file_status: FileStatus,
    text_buf: String,
    is_open_window: bool,
    is_save_window: bool,
    is_save_as_window:bool,
    file_to_open: String,
}

impl Default for CodeShare {
    fn default() -> Self {
        Self {
            file_status: FileStatus::default(),
            text_buf: "Hello World!".to_owned(),
            is_open_window: false,
            is_save_window: false,
            is_save_as_window: false,
            file_to_open: String::from(std::env::var("HOME").unwrap()),
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
        let Self { file_status, text_buf, is_open_window, is_save_window, is_save_as_window, file_to_open, } = self;


        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Open").clicked(){
                        *is_open_window = true;
                    }
                    if ui.button("Save").clicked() {
                        match file_status.is_new() {
                            true => {
                                *is_save_as_window = true;
                            }
                            false => *is_save_window = true
                        };
                    }
                    if ui.button("Save As").clicked() {
                        *is_save_as_window = true;
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
                        file_status.set_path(file_to_open);
                        *text_buf = match file_status.get_contents() {
                            Ok(c) => c,
                            Err(e) => panic!("Error: {}",e)
                        };
                        *is_open_window = false;
                    }
                    if ui.button("Close").clicked() {
                        *is_open_window = false;
                    }   
                });
            });
        }

        if *is_save_window {
            egui::Window::new("Save Status").show(ctx, |ui| {
                match file_status.save_file(text_buf) {
                    Ok(_) => {
                        ui.label("Save Successful");
                        if ui.button("OK").clicked() {
                            *is_save_window = false;
                            *is_save_as_window = false;
                        }
                    },
                    Err(e) => {
                        let error = format!("Save failed: {}", e);
                        ui.label(error);
                        if ui.button("OK").clicked() {
                            *is_save_window = false;
                            *is_save_as_window = true;
                        }
                    }
                };
            });
        }

        if *is_save_as_window {
            egui::Window::new("Save As").show(ctx, |ui| {
                ui.text_edit_singleline(file_to_open);
                ui.horizontal( |ui| {
                    if ui.button("Save").clicked() {
                        file_status.set_path(file_to_open);
                        *is_save_window = true;
                        
                    }
                    if ui.button("Quit").clicked() {
                        *is_save_as_window = false;
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
