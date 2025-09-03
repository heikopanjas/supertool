use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod id3v2_3_dissector;
mod id3v2_4_dissector;
mod id3v2_dissector;
mod id3v2_tools;
mod isobmff_dissector;

use id3v2_dissector::dissect_id3v2;
use isobmff_dissector::dissect_isobmff;

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

#[derive(Debug)]
enum MediaType {
    Id3v2,
    IsoMp4,
    Unknown,
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

    // Read file header to determine format
    let mut file = File::open(file_path)?;
    let mut header = [0u8; 12];
    file.read_exact(&mut header)?;

    let media_type = detect_media_type(&header);

    // Print file info
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;
    writeln!(&mut stdout, "Analyzing file: {}", file_path.display())?;
    stdout.reset()?;

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(&mut stdout, "Detected format: ")?;
    stdout.reset()?;

    match media_type {
        | MediaType::Id3v2 => {
            writeln!(&mut stdout, "ID3v2 (MP3)")?;
            dissect_id3v2(&mut file)?;
        }
        | MediaType::IsoMp4 => {
            writeln!(&mut stdout, "ISO Base Media File Format (MP4)")?;
            dissect_isobmff(&mut file)?;
        }
        | MediaType::Unknown => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            writeln!(&mut stdout, "Unknown (unsupported format)")?;
            stdout.reset()?;
        }
    }

    Ok(())
}

fn detect_media_type(header: &[u8; 12]) -> MediaType {
    // ID3v2 detection - look for ID3v2 header or MPEG sync
    if header[0..3] == [0x49, 0x44, 0x33] {
        // "ID3"
        return MediaType::Id3v2;
    }

    // Check for MPEG sync pattern (0xFF followed by 0xFB, 0xFA, 0xF3, 0xF2)
    if header[0] == 0xFF && (header[1] & 0xE0) == 0xE0 {
        return MediaType::Id3v2;
    }

    // ISO Base Media File Format detection - look for ftyp box
    if header[4..8] == [0x66, 0x74, 0x79, 0x70] {
        // "ftyp"
        return MediaType::IsoMp4;
    }

    MediaType::Unknown
}
