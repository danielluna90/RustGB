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
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
            ..Default::default()
    };

    eframe::run_native("RustGB", options, Box::new(|_| {
        Box::<RustGB>::default()
    }))
}

struct RustGB {
    args: Args,
    cpu: CPU
}

impl Default for RustGB {
    fn default() -> Self {
        Self {
            args: Args::parse(),
            cpu: CPU::new()
        }
    }
}

impl eframe::App for RustGB {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RustGB - Debug");
            ui.label(format!("Filepath: {}, CPU Registers: {}", self.args.filename, self.cpu.registers));
        });
    }
}