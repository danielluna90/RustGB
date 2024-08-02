use clap::Parser;

mod cpu;
mod debugger;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CLI {
    /// Sets the ROM file to load.
    filename: String,

    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let cli: CLI = CLI::parse();

    if cli.debug {
        debugger::Debugger {};
    }
}
