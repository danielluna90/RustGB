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
}

impl Default for RustGB {
    fn default() -> Self {
        Self {
            args: Args::parse(),
            cpu: CPU::new(),
        }
    }
}

impl eframe::App for RustGB {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("Debug Tools").show(ctx, |ui| {
            ui.heading("RustGB - Debug Tools");
            ui.label(format!("CPU Registers:\n{}", self.cpu.registers));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("The Game Window"));
        });
    }
}
