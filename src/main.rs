use std::path::Path;

use clap::Parser;
use eframe::egui::{self, Modifiers};
use egui_extras::{Column, TableBuilder};

use crate::cpu::CPU;

pub mod cpu;
pub mod registers;

#[derive(Parser)]
#[command(name = "RustGB")]
#[command(version = "0.1")]
struct Args {
    filename: String,

    #[arg(long, default_value_t = false)]
    headless: bool,
}

fn main() -> Result<(), eframe::Error> {
    let args: Args = Args::parse();

    if args.headless && !Path::new(&args.filename).is_file() {
        print!("File at filepath {} does not exist", args.filename);

        return Ok(());
    }

    if !args.headless {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
            ..Default::default()
        };

        eframe::run_native("RustGB", options, Box::new(|_| Box::<RustGB>::default()))
    } else {
        print!("Headless mode is not implemented so far.");

        Ok(())
    }
}

struct WindowState {
    // Tools
    register_view: bool,

    // Settings
    egui_settings: bool,
}

impl WindowState {
    fn default() -> Self {
        Self {
            register_view: false,
            egui_settings: false,
        }
    }
}

struct RustGB {
    args: Args,
    cpu: CPU,

    state: WindowState,
}

impl Default for RustGB {
    fn default() -> Self {
        Self {
            args: Args::parse(),
            cpu: CPU::new(),

            state: WindowState::default(),
        }
    }
}

impl eframe::App for RustGB {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let open_file_shortcut =
                    egui::KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::O);

                if ui.input_mut(|i| i.consume_shortcut(&open_file_shortcut)) {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.args.filename = Some(path.display().to_string()).unwrap();
                    }

                    ui.close_menu();
                }

                ui.menu_button("File", |ui| {
                    ui.style_mut().wrap = Some(false);

                    if ui
                        .add(
                            egui::Button::new("Open File...")
                                .shortcut_text(ui.ctx().format_shortcut(&open_file_shortcut)),
                        )
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.args.filename = Some(path.display().to_string()).unwrap();
                        }

                        ui.close_menu();
                    }
                });

                ui.menu_button("Tools", |ui| {
                    ui.style_mut().wrap = Some(false);

                    if ui.add(egui::Button::new("Register View")).clicked() {
                        self.state.register_view = !self.state.register_view;
                        ui.close_menu();
                    }
                });

                ui.menu_button("Settings", |ui| {
                    ui.style_mut().wrap = Some(false);

                    if ui.add(egui::Button::new("eFrame Settings")).clicked() {
                        self.state.egui_settings = !self.state.egui_settings;
                        ui.close_menu();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Playing Game: {}", self.args.filename));
        });

        egui::Window::new("Register View")
            .open(&mut self.state.register_view)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                TableBuilder::new(ui)
                    .cell_layout(egui::Layout::centered_and_justified(
                        egui::Direction::LeftToRight,
                    ))
                    .column(Column::exact(75.0).resizable(false))
                    .column(Column::exact(75.0).resizable(false))
                    .header(25.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("Register");
                        });

                        header.col(|ui| {
                            ui.heading("Value");
                        });
                    })
                    .body(|mut body| {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label("A");
                            });
                            row.col(|ui| {
                                ui.label(format!("{:#04x}", self.cpu.registers.a));
                            });
                        });
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label("B");
                            });
                            row.col(|ui| {
                                ui.label(format!("{:#04x}", self.cpu.registers.b));
                            });
                        });
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label("C");
                            });
                            row.col(|ui| {
                                ui.label(format!("{:#04x}", self.cpu.registers.c));
                            });
                        });
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label("D");
                            });
                            row.col(|ui| {
                                ui.label(format!("{:#04x}", self.cpu.registers.d));
                            });
                        });
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label("E");
                            });
                            row.col(|ui| {
                                ui.label(format!("{:#04x}", self.cpu.registers.e));
                            });
                        });
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label("H");
                            });
                            row.col(|ui| {
                                ui.label(format!("{:#04x}", self.cpu.registers.h));
                            });
                        });
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label("L");
                            });
                            row.col(|ui| {
                                ui.label(format!("{:#04x}", self.cpu.registers.l));
                            });
                        });
                    });
            });

        egui::Window::new("eFrame Settings")
            .open(&mut self.state.egui_settings)
            .vscroll(true)
            .show(ctx, |ui| {
                ctx.settings_ui(ui);
            });
    }
}
