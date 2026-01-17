use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cue")]
#[command(about = "Play categorized audio cues from a sound library")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Play a random sound from a category
    Play {
        /// The category to play from
        category: String,

        /// Volume level 0-100+
        #[arg(short, long, default_value = "100")]
        volume: u32,
    },

    /// List available categories or sounds
    List {
        /// Category to list sounds from (omit for all categories)
        category: Option<String>,
    },

    /// Play a specific sound file
    Preview {
        /// The category containing the sound
        category: String,

        /// The sound name (without extension)
        sound: String,

        /// Volume level 0-100+
        #[arg(short, long, default_value = "100")]
        volume: u32,
    },
}
