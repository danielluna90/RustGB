use std::path::Path;

use clap::Parser;
use eframe::egui;

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

struct RustGB {
    args: Args,
    cpu: CPU,

    settings: bool,
}

impl Default for RustGB {
    fn default() -> Self {
        Self {
            args: Args::parse(),
            cpu: CPU::new(),

            settings: true,
        }
    }
}

impl eframe::App for RustGB {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Tools", |ui| {
                    ui.set_min_width(220.0);
                    ui.style_mut().wrap = Some(false);

                    if ui.add(egui::Button::new("Register View")).clicked() {
                        self.settings = !self.settings;
                        ui.close_menu();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("The Game Window"));
        });

        egui::Window::new("eFrame Settings")
            .open(&mut self.settings)
            .vscroll(true)
            .show(ctx, |ui| {
                ctx.settings_ui(ui);
            });

        egui::Window::new("Debug Tools")
            .open(&mut self.settings)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label(format!("CPU Registers:\n{}", self.cpu.registers));
            });
    }
}
