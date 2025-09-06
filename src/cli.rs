use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "supertool")]
#[command(about = "A versatile media file analysis tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Debug and analyze media files (ID3v2/MP3, ISO BMFF/MP4)
    Debug {
        /// Path to the media file to analyze
        file: PathBuf,
    },
}
