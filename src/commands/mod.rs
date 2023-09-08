use clap::Subcommand;
pub use predict::*;
pub use today::*;

mod predict;
mod today;

#[derive(Subcommand)]
pub enum Command {
    /// Get the weather for today
    Today,
    /// Predict the weather for a given city
    Predict {
        /// The date to predict the weather for (format: YYYY-MM-DD)
        date: String,
    },
}
