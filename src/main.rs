use crate::cli::{Cli, Commands, DebugOptions};
use clap::Parser;
use std::fs::File;
use std::path::PathBuf;

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
        | Commands::Debug { file, header, frames, all } => {
            let options = DebugOptions::from_flags(header, frames, all);
            dissect_file(&file, &options)?;
        }
    }

    Ok(())
}

fn dissect_file(file_path: &PathBuf, options: &DebugOptions) -> Result<(), Box<dyn std::error::Error>> {
    // Open file
    let mut file = File::open(file_path)?;

    // Build appropriate dissector based on file content
    let builder = DissectorBuilder::new();
    let dissector = builder.build_for_file(&mut file)?;

    // Print file info
    println!("Analyzing file: {}", file_path.display());
    println!("Detected format: {} ({})", dissector.media_type(), dissector.name());

    // Perform dissection with options
    dissector.dissect_with_options(&mut file, options)?;

    Ok(())
}
