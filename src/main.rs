use clap::Parser;
use opentdd_rs::cli::Cli;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(cmd) => match cmd {},
        None => {
			
		}
    }
}
