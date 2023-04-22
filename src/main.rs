use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, about, version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Server,
    Reciever,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Server) => server::run(),
        Some(Commands::Reciever) => reciever::run(),
        None => println!("No command given"),
    }
}
