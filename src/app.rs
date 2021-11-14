use eframe::{egui::{self, ScrollArea}, epi};
use crate::file::*;



pub struct CodeShare {
    file_status: FileStatus,
    text_buf: String,
    is_buffer_saved: bool,
    is_open_window: bool,
    is_save_window: bool,
    is_save_as_window:bool,
    is_unsaved_window: bool,
    is_error: bool,
    err_msg: Option<String>,
}

impl Default for CodeShare {
    fn default() -> Self {
        Self {
            file_status: FileStatus::default(),
            text_buf: String::new(),
            is_buffer_saved: false,
            is_open_window: false,
            is_save_window: false,
            is_save_as_window: false,
            is_unsaved_window: false,
            is_error: false,
            err_msg: None,
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
        /*
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
        */
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
        let Self {
            file_status,
            text_buf,
            is_buffer_saved,
            is_open_window,
            is_save_window,
            is_save_as_window,
            is_unsaved_window,
            is_error,
            err_msg,
        } = self;


        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("New").clicked() {
                        match *is_buffer_saved {
                            true => text_buf.clear(),
                            false => {
                                *is_unsaved_window = true;
                            }
                        }
                    }
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
            match file_status.open_file() {
                Ok(contents) => {
                    *text_buf = contents;
                    *is_open_window = false;
                },
                Err(e) => {
                    *err_msg = Some(e.to_string());
                    *is_error = true;
                    *is_open_window = false;
                }
            };
        }
        // File save status window
        if *is_save_window {
            egui::Window::new("Save Status").show(ctx, |ui| {
                match file_status.save_file(text_buf) {
                    Ok(_) => {
                        ui.label("Save Successful");
                        if ui.button("OK").clicked() {
                            *is_buffer_saved = true;
                            *is_save_window = false;
                            *is_save_as_window = false;
                        }
                    },
                    Err(e) => {
                        let error = format!("Save failed: {}", e);
                        ui.label(error);
                        if ui.button("OK").clicked() {
                            *is_save_window = false;
                        }
                    }
                };
            });
        }
        //   Save as window
        if *is_save_as_window {
            match file_status.save_file_as(text_buf) {
                Ok(_) => *is_buffer_saved = true,
                Err(e) => {
                    *err_msg = Some(e.to_string());
                    *is_error = true;
                }
            };
            *is_save_as_window = false;
        }
        //  File not saved popup
        if *is_unsaved_window {
            egui::Window::new("File Not Saved").show(ctx, |ui| {
                ui.label("Current file has not been saved");
                ui.horizontal( |ui| {
                    if ui.button("Save").clicked() {
                        *is_save_window = true;
                        *is_unsaved_window = false;
                    }
                    if ui.button("Save As").clicked() {
                        *is_save_as_window = true;
                        *is_unsaved_window = false;
                    }
                    if ui.button("Continue without saving").clicked() {
                        *is_unsaved_window = false;
                        text_buf.clear();
                    }   
                });
            });
        }

        if *is_error {
            egui::Window::new("Error").show(ctx, |ui| {
                let error = match err_msg {
                    Some(msg) => msg.clone(),
                    None => String::from("Unknown Error")
                };
                ui.label(error);
                if ui.button("Close").clicked() {
                    *is_error = false;
                    *err_msg = None;
                }
            });
        }

        egui::CentralPanel::default().frame(egui::Frame::none().corner_radius(0.0)).show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui|{
                let editor = ui.add_sized(ui.available_size(),
                egui::TextEdit::multiline(text_buf)
                    .text_style(egui::TextStyle::Monospace)
                    .code_editor()
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                );
                if editor.changed() {
                    *is_buffer_saved = false;
                }

                // ui.add_sized(ui.available_size(),
                //     egui::TextEdit::multiline(text_buf)
                //         .text_style(egui::TextStyle::Monospace)
                //         .code_editor()
                //         .lock_focus(true)
                //         .desired_width(f32::INFINITY)
                // )
            });
        });

    }
}
