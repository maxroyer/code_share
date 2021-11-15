use eframe::epi;
use eframe::egui;
use crate::file::*;
use crate::app_config::AppConfig;

pub struct CodeShare {
    config: AppConfig,
    
    #[cfg_attr(feature = "persistence", serde(skip))]
    file_status: FileStatus,
    text_buf: String,
    active_popup: Popup,
    err_msg: Option<String>,
    status_msg: Option<&'static str>,
}

impl CodeShare {
    fn change_app_font_size(ctx: &egui::CtxRef, size: f32) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.family_and_size.insert(
            egui::TextStyle::Monospace,
            (egui::FontFamily::Monospace, size)
        );
        ctx.set_fonts(fonts);
    }
}

impl Default for CodeShare {
    fn default() -> Self {
        Self {
            config: AppConfig::default(),
            file_status: FileStatus::default(),
            text_buf: String::new(),
            active_popup: Popup::None,
            err_msg: None,
            status_msg: Some("code_share loaded"),
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
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // let mut fonts = egui::FontDefinitions::default();
        // fonts.family_and_size.insert(
        //     egui::TextStyle::Monospace,
        //     (egui::FontFamily::Monospace, 15.0)
        // );
        // ctx.set_fonts(fonts);
        
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
        let Self {
            config,
            file_status,
            text_buf,
            active_popup,
            err_msg,
            status_msg,
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
                        *active_popup = Popup::SaveFile;
                    }
                    if ui.button("Save As").clicked() {
                        *active_popup = Popup::SaveAs
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                egui::menu::menu(ui, "Edit", |ui| {
                    ui.label("to be built");
                });
                egui::menu::menu(ui, "View", |ui| {
                    ui.horizontal(|ui| {
                        if ui.button(" - ").clicked() {
                           config.dec_font_size();
                           CodeShare::change_app_font_size(ctx, config.get_font_size());
                        }
                        let font_size_str = format!("Font Size: {}", config.get_font_size());
                        ui.label(font_size_str);
                        if ui.button(" + ").clicked() {
                            config.inc_font_size();
                            CodeShare::change_app_font_size(ctx, config.get_font_size());
                        }
                    });
                });
            });
        });

        //  Save File "popup"
        if *active_popup == Popup::SaveFile {
            match file_status.is_new() {
                true => *active_popup = Popup::SaveAs,
                false => {
                    match file_status.save_file(text_buf) {
                        Ok(_) => {
                            *active_popup = Popup::None;
                            *status_msg = Some("Save Successful");
                        },
                        Err(e) => {
                            let error = format!("Save failed: {}", e);
                            *err_msg = Some(error);
                            *active_popup = Popup::Error;
                            *status_msg = Some("Save Failed")
                        }
                    };
                }
            };
        }
        //  Open file popup
        if *active_popup == Popup::OpenFile {
            match file_status.open_file() {
                Ok(Some(contents)) => {
                    *text_buf = contents;
                    *active_popup = Popup::None;
                    *status_msg = Some("Open Successful");
                },
                Ok(None) => *active_popup = Popup::None,
                Err(e) => {
                    *err_msg = Some(e.to_string());
                    *active_popup = Popup::Error;
                }
            };
        }
        //   Save as popup
        if *active_popup == Popup::SaveAs {
            match file_status.save_file_as(text_buf) {
                Ok(Some(_)) => *active_popup = Popup::None,
                Ok(None) => *active_popup = Popup::None,
                Err(e) => {
                    *err_msg = Some(e.to_string());
                    *active_popup = Popup::Error;
                }
            };
            
        }
        //  File not saved popup
        if *active_popup == Popup::FileNotSaved {
            egui::Window::new("File Not Saved").collapsible(false).show(ctx, |ui| {
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
        if *active_popup == Popup::Error {
            egui::Window::new("Error").collapsible(false).show(ctx, |ui| {
                let error = match err_msg {
                    Some(msg) => msg.clone(),
                    None => String::from("Unknown Error")
                };
                ui.label(error);
                if ui.button("Close").clicked() {
                    *active_popup = Popup::None;
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
                        .frame(false)
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

        egui::TopBottomPanel::bottom("info bar").frame(egui::Frame::none().corner_radius(0.0).fill(egui::Color32::WHITE)).show(ctx, |ui| {
            let indicator = match file_status.is_unsaved() {true => "*", false => ""};
            let mut title_line = format!("{}{}", file_status.get_path_string(), indicator);
            ui.horizontal(|ui| {
                ui.add(egui::widgets::TextEdit::singleline(&mut title_line)
                    .code_editor()
                    .frame(false)
                    .interactive(false)
                    .text_color(egui::Color32::BLACK)
                );
                
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    let mut msg_to_display = match status_msg {
                        Some(msg) => msg,
                        None => ""
                    };
                    ui.add(egui::widgets::TextEdit::singleline(&mut msg_to_display)
                        .code_editor()
                        .frame(false)
                        .interactive(false)
                        .text_color(egui::Color32::BLACK)
                        .desired_width(150.0)
                    );
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
