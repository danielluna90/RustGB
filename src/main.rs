use clap::Parser;
use crate::cpu::CPU;

pub mod cpu;
pub mod registers;

#[derive(Parser)]
#[command(name = "RustGB")]
#[command(version = "0.1")]
struct Args {
    filename: String,
}

fn main() {
    let args: Args = Args::parse();

    println!("Filename: {}", args.filename);

    let cpu: CPU = CPU::new();

    println!("{}", cpu.registers);
}
