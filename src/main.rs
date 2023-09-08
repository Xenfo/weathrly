#![feature(exclusive_range_pattern)]

use clap::Parser;

mod api;
mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The city to get weather for
    city: String,

    #[command(subcommand)]
    command: commands::Command,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        commands::Command::Today => {
            commands::today(args.city).await;
        }
        commands::Command::Predict { date } => {
            commands::predict(args.city, date).await;
        }
    }
}
