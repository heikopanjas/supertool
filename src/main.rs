use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod dissector;
mod id3v2_3_dissector;
mod id3v2_4_dissector;
mod id3v2_tools;
mod isobmff_dissector;

use dissector::DissectorBuilder;

#[derive(Parser)]
#[command(name = "supertool")]
#[command(about = "A versatile media file analysis tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Dissect and analyze media files (ID3v2/MP3, ISO BMFF/MP4)
    Dissect {
        /// Path to the media file to analyze
        file: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        | Commands::Dissect { file } => {
            dissect_file(&file)?;
        }
    }

    Ok(())
}

fn dissect_file(file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    // Open file
    let mut file = File::open(file_path)?;

    // Build appropriate dissector based on file content
    let builder = DissectorBuilder::new();
    let dissector = builder.build_for_file(&mut file)?;

    // Print file info
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;
    writeln!(&mut stdout, "Analyzing file: {}", file_path.display())?;
    stdout.reset()?;

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "Detected format: {} ({})", dissector.media_type(), dissector.name())?;
    stdout.reset()?;

    // Perform dissection
    dissector.dissect(&mut file)?;

    Ok(())
}
