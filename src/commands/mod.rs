use clap::Subcommand;
pub use predict::*;

mod predict;

#[derive(Subcommand)]
pub enum Command {
    /// Predict the weather for a given city
    Predict {
        /// The city to predict the weather for
        city: String,
        /// The date to predict the weather for (format: YYYY-MM-DD)
        date: String,
    },
}
