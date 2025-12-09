use clap::{Parser, Subcommand};

mod one;
mod util;
use one::run as run_one;
mod two;
use two::run as run_two;
mod three;
use three::run as run_three;
mod four;
use four::run as run_four;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, action)]
    debug: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    One {},
    Two {},
    Three {},
    Four {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::One {} => run_one(),
        Commands::Two {} => run_two(),
        Commands::Three {} => run_three(),
        Commands::Four {} => run_four(),
    };
}
