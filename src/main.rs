use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "clap-demo")]
#[command(about = "Bare bones Clap example", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hello { name: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello { name } => {
            println!("Hello, {}!", name);
        }
    }
}
