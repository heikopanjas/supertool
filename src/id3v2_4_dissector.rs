use crate::id3v2_tools::*;
use crate::media_dissector::MediaDissector;
use std::fs::File;
use std::io::{Read, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// ID3v2.4 dissector for MP3 files
pub struct Id3v24Dissector;

impl MediaDissector for Id3v24Dissector {
    fn media_type(&self) -> &'static str {
        "ID3v2.4"
    }

    fn dissect(&self, file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
        dissect_id3v2_4_file(file)
    }

    fn can_handle(&self, header: &[u8]) -> bool {
        // Check for ID3v2.4 specifically
        if let Some((major, _minor)) = detect_id3v2_version(header) {
            return major == 4;
        }

        false // Don't fall back to MPEG sync for v2.4 since v2.3 should handle that
    }

    fn name(&self) -> &'static str {
        "ID3v2.4 Dissector"
    }
}

/// Dissect an ID3v2.4 file from the beginning
pub fn dissect_id3v2_4_file(file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    // Read and parse ID3v2 header
    if let Some((major, minor, flags, size)) = read_id3v2_header(file)? {
        if major == 4 {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))?;
            writeln!(&mut stdout, "\nID3v2 Header Found:")?;
            stdout.reset()?;

            writeln!(&mut stdout, "  Version: 2.{}.{}", major, minor)?;
            writeln!(&mut stdout, "  Flags: 0x{:02X}", flags)?;

            // Interpret header flags
            if flags != 0 {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
                write!(&mut stdout, "    ")?;
                let mut flag_parts = Vec::new();
                if flags & 0x80 != 0 {
                    flag_parts.push("unsynchronisation");
                }
                if flags & 0x40 != 0 {
                    flag_parts.push("extended_header");
                }
                if flags & 0x20 != 0 {
                    flag_parts.push("experimental");
                }
                if flags & 0x10 != 0 {
                    flag_parts.push("footer_present");
                }
                if !flag_parts.is_empty() {
                    writeln!(&mut stdout, "Active: {}", flag_parts.join(", "))?;
                }
                stdout.reset()?;
            }

            writeln!(&mut stdout, "  Tag Size: {} bytes", size)?;

            if size > 0 && size < 1_000_000 {
                // Basic sanity check
                dissect_id3v2_4(file, size, flags)?;
            }
        } else {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            writeln!(&mut stdout, "  Expected ID3v2.4, found version 2.{}", major)?;
            stdout.reset()?;
        }
    } else {
        writeln!(&mut stdout, "No ID3v2 header found")?;
    }

    Ok(())
}

pub fn dissect_id3v2_4(file: &mut File, tag_size: u32, flags: u8) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut buffer = vec![0u8; tag_size as usize];
    file.read_exact(&mut buffer)?;

    // Handle unsynchronization if flag is set
    let unsync_flag = flags & 0x80 != 0; // Bit 7
    if unsync_flag {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        writeln!(&mut stdout, "  Unsynchronization detected - removing sync bytes")?;
        stdout.reset()?;
        buffer = remove_unsynchronization(&buffer);
    }

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))?;
    writeln!(&mut stdout, "\nID3v2.4 Frames:")?;
    stdout.reset()?;

    // Check for extended header
    let mut frame_start = 0;
    if flags & 0x40 != 0 {
        // Extended header flag
        if buffer.len() >= 4 {
            // ID3v2.4 uses synchsafe integers for extended header size
            let extended_size = decode_synchsafe_int(&buffer[0..4]);
            frame_start = 4 + extended_size as usize;
            writeln!(&mut stdout, "  Extended header found (size: {} bytes)", extended_size)?;
        }
    }

    let mut pos = frame_start;
    while pos + 10 <= buffer.len() {
        // ID3v2.4 frame header: 4 bytes ID + 4 bytes size + 2 bytes flags
        let frame_id = std::str::from_utf8(&buffer[pos..pos + 4]).unwrap_or("????");

        // Stop if we hit padding (null bytes)
        if frame_id.starts_with('\0') {
            writeln!(&mut stdout, "  Reached padding section")?;
            break;
        }

        if frame_id.chars().all(|c| c.is_ascii_alphanumeric()) {
            // ID3v2.4 uses synchsafe integers for frame size
            let frame_size = decode_synchsafe_int(&buffer[pos + 4..pos + 8]);
            let frame_flags = u16::from_be_bytes([buffer[pos + 8], buffer[pos + 9]]);

            if frame_size > 0 && frame_size < (buffer.len() - pos - 10) as u32 {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                write!(&mut stdout, "  Frame: {}", frame_id)?;
                stdout.reset()?;
                write!(&mut stdout, " (size: {} bytes", frame_size)?;

                // Interpret frame flags for ID3v2.4
                interpret_id3v2_4_frame_flags(frame_flags)?;
                writeln!(&mut stdout, ")")?;

                // Parse common frame content preview
                if frame_size > 1 && pos + 10 + frame_size as usize <= buffer.len() {
                    parse_frame_content_preview(&buffer[pos + 10..pos + 10 + frame_size as usize], frame_id)?;
                }

                pos += 10 + frame_size as usize;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(())
}
