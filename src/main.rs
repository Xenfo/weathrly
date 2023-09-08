use clap::Parser;

mod api;
mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: commands::Command,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        commands::Command::Predict { city, date } => {
            commands::predict(city, date).await;
        }
    }
}
