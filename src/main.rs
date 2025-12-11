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
mod five;
use five::run as run_five;
mod six;
use six::run as run_six;
mod seven;
use seven::run as run_seven;

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
    All {},
    One {},
    Two {},
    Three {},
    Four {},
    Five {},
    Six {},
    Seven {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::All {} => {
            run_one();
            run_two();
            run_three();
            run_four();
            run_five();
            run_six();
            run_seven();
        }
        Commands::One {} => run_one(),
        Commands::Two {} => run_two(),
        Commands::Three {} => run_three(),
        Commands::Four {} => run_four(),
        Commands::Five {} => run_five(),
        Commands::Six {} => run_six(),
        Commands::Seven {} => run_seven(),
    };
}
