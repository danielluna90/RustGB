use clap::Parser;

pub mod cpu;
pub mod registers;

#[derive(Parser)]
#[command(name = "RustGB")]
#[command(version = "0.1")]
struct Args {
    filename: String,
}

fn main() {
    let args = Args::parse();

    println!("Filename: {}", args.filename);

    println!("Hello, World!");
}
