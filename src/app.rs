use eframe::epi;
use eframe::egui;
use crate::file::*;

pub struct CodeShare {
    file_status: FileStatus,
    text_buf: String,
    active_popup: Popup,
    is_error: bool,
    err_msg: Option<String>,
}

impl Default for CodeShare {
    fn default() -> Self {
        Self {
            file_status: FileStatus::default(),
            text_buf: String::new(),
            active_popup: Popup::None,
            is_error: false,
            err_msg: None,
        }
    }
}

impl epi::App for CodeShare {
    fn name(&self) -> &str {
        "code_share"
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
            active_popup,
            is_error,
            err_msg,
        } = self;


        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("New").clicked() {
                        match file_status.is_unsaved() {
                            false => {
                                text_buf.clear();
                                file_status.reset();
                            },
                            true => {
                                *active_popup = Popup::FileNotSaved;
                            }
                        }
                    }
                    if ui.button("Open").clicked(){
                        *active_popup = Popup::OpenFile;
                    }
                    if ui.button("Save").clicked() {
                        match file_status.is_new() {
                            true => *active_popup = Popup::SaveAs,
                            false => *active_popup = Popup::SaveFile
                        };
                    }
                    if ui.button("Save As").clicked() {
                        *active_popup = Popup::SaveAs
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
            let indicator = match file_status.is_unsaved() {true => "*", false => ""};
                let title_line = format!("{}{}", file_status.get_path_string(), indicator);
                ui.add(egui::widgets::Label::new(title_line)
                    .monospace()
                    .strong()
                    .text_color(egui::Color32::BLACK)
                    .background_color(egui::Color32::LIGHT_GRAY)
                );
        });

        //  Open file popup
        if *active_popup == Popup::OpenFile {
            match file_status.open_file() {
                Ok(Some(contents)) => {
                    *text_buf = contents;
                    *active_popup = Popup::None;
                },
                Ok(None) => *active_popup = Popup::None,
                Err(e) => {
                    *err_msg = Some(e.to_string());
                    *is_error = true;
                    *active_popup = Popup::Error;
                }
            };
        }
        // File save popup
        if *active_popup == Popup::SaveFile {
            egui::Window::new("Save Status").show(ctx, |ui| {
                match file_status.save_file(text_buf) {
                    Ok(_) => {
                        ui.label("Save Successful");
                        if ui.button("OK").clicked() { *active_popup = Popup::None; }
                    },
                    Err(e) => {
                        let error = format!("Save failed: {}", e);
                        ui.label(error);
                        if ui.button("OK").clicked() {
                            *active_popup = Popup::None;
                        }
                    }
                };
            });
        }
        //   Save as popup
        if *active_popup == Popup::SaveAs {
            match file_status.save_file_as(text_buf) {
                Ok(Some(_)) => *active_popup = Popup::None,
                Ok(None) => *active_popup = Popup::None,
                Err(e) => {
                    *err_msg = Some(e.to_string());
                    *is_error = true;
                }
            };
            
        }
        //  File not saved popup
        if *active_popup == Popup::FileNotSaved {
            egui::Window::new("File Not Saved").show(ctx, |ui| {
                ui.label("Current file has not been saved");
                ui.horizontal( |ui| {
                    if ui.button("Save").clicked() {
                        *active_popup = Popup::SaveFile;
                    }
                    if ui.button("Save As").clicked() {
                        *active_popup = Popup::SaveAs;
                    }
                    if ui.button("Continue without saving").clicked() {
                        text_buf.clear();
                        file_status.reset();
                        *active_popup = Popup::None;
                    }   
                });
            });
        }
        //  Error Popup
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
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    let mut lines_str = get_line_num_str(text_buf.lines().count());
                    ui.add_sized([40.0, ui.available_height()], egui::TextEdit::multiline(&mut lines_str)
                        .desired_width(40.0)
                        .code_editor()
                        .interactive(false)
                    );
                    let editor = ui.add_sized(ui.available_size(),
                        egui::TextEdit::multiline(text_buf)
                            .text_style(egui::TextStyle::Monospace)
                            .code_editor()
                            .lock_focus(true)
                            .desired_width(f32::INFINITY)
                    );
                    if editor.changed() {
                        file_status.set_unsaved(true);
                    }
                });
            });  
        });
    }
}

fn get_line_num_str(count: usize) -> String {
    let mut lines_str = String::new();
    for num in 1..=count {
        match num {
            num if num < 10 => {
                lines_str.push_str("   ");
                lines_str.push_str(&num.to_string());
                lines_str.push('\n');
            },
            num if num < 100 => {
                lines_str.push_str("  ");
                lines_str.push_str(&num.to_string());
                lines_str.push('\n');
            },
            num if num < 1000 => {
                lines_str.push_str(" ");
                lines_str.push_str(&num.to_string());
                lines_str.push('\n');
            },
            num if num >= 1000 => {
                lines_str.push_str(&num.to_string());
                lines_str.push('\n');
            }
            _ => (),
        }
    }
    lines_str.push_str("   ~");
    lines_str
}

#[derive(PartialEq)]
enum Popup {
    OpenFile,
    SaveFile,
    SaveAs,
    FileNotSaved,
    Error,
    None,
}
