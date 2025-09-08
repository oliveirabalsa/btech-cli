use clap::{Parser, Subcommand};
use uuid::Uuid;
use lipsum::lipsum;

#[derive(Parser)]
#[command(name = "btech", about = "A CLI for managing btech projects")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
struct LoremArgs {
    #[arg(short, long, default_value_t = 10)]
    count: u32,
}

#[derive(Subcommand)]
enum Commands {
    Uuid,
    Lorem(LoremArgs),
}

fn print_uuid() {
    let new_uuid = Uuid::new_v4();
    println!("{}", new_uuid);
}

fn print_lorem(count: u32) {
    let lorem = lipsum(count as usize);
    println!("{}", lorem);
}



fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Uuid => {
            print_uuid();
        }
        Commands::Lorem(args) => {
            print_lorem(args.count);
        }
    }
}

