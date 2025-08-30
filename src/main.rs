mod initialize;
use clap::{Subcommand, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init
}
fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Init => {
            initialize::init();
        }
    }
}
