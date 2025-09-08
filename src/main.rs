use clap::{Parser, Subcommand};

mod commands;
mod utils;

use commands::{uuid, lorem};

#[derive(Parser)]
#[command(
    name = "btech",
    about = "A powerful CLI tool for developers providing quick access to common utilities",
    version,
    author
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Uuid,
    Lorem(lorem::LoremArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Uuid => {
            uuid::execute();
        }
        Commands::Lorem(args) => {
            lorem::execute(&args);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let cli = Cli::parse_from(["btech", "uuid"]);
        match cli.command {
            Commands::Uuid => {}
            _ => panic!("Expected Uuid command"),
        }
    }
}

