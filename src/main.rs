use crate::cli::{Cli, Commands};
use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod cli;
mod dissector_builder;
mod id3v2_3_dissector;
mod id3v2_4_dissector;
mod id3v2_attached_picture_frame;
mod id3v2_chapter_frame;
mod id3v2_comment_frame;
mod id3v2_frame;
mod id3v2_table_of_contents_frame;
mod id3v2_text_encoding;
mod id3v2_text_frame;
mod id3v2_tools;
mod id3v2_unique_file_id_frame;
mod id3v2_url_frame;
mod id3v2_user_text_frame;
mod id3v2_user_url_frame;
mod isobmff_dissector;
mod media_dissector;
mod unknown_dissector;

use dissector_builder::DissectorBuilder;

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
