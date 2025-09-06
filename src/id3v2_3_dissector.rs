use crate::id3v2_frame::Id3v2Frame;
use crate::id3v2_tools::*;
use crate::media_dissector::MediaDissector;
use std::fs::File;
use std::io::{Read, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// ID3v2.3 dissector for MP3 files
pub struct Id3v23Dissector;

/// Parse an ID3v2.3 frame from raw buffer data
pub fn parse_id3v2_3_frame(buffer: &[u8], pos: usize) -> Option<Id3v2Frame> {
    if pos + 10 > buffer.len() {
        return None;
    }

    let frame_id = String::from_utf8_lossy(&buffer[pos..pos + 4]).to_string();

    // Stop if we hit padding (null bytes)
    if frame_id.starts_with('\0') || !frame_id.chars().all(|c| c.is_ascii_alphanumeric()) {
        return None;
    }

    // Check if this is a valid ID3v2.3 frame ID
    if !crate::id3v2_tools::is_valid_frame_for_version(&frame_id, 3) {
        return None;
    }

    // ID3v2.3 uses regular big-endian integers (not synchsafe)
    let frame_size = u32::from_be_bytes([buffer[pos + 4], buffer[pos + 5], buffer[pos + 6], buffer[pos + 7]]);
    let frame_flags = u16::from_be_bytes([buffer[pos + 8], buffer[pos + 9]]);

    if frame_size == 0 || frame_size > (buffer.len() - pos - 10) as u32 {
        return None;
    }

    let data = buffer[pos + 10..pos + 10 + frame_size as usize].to_vec();

    let mut frame = Id3v2Frame::new(frame_id.clone(), frame_size, frame_flags, data);

    // Parse the frame content using the new typed system (ID3v2.3)
    let _ = frame.parse_content(3); // Ignore parsing errors, keep raw data

    Some(frame)
}

impl MediaDissector for Id3v23Dissector {
    fn media_type(&self) -> &'static str {
        "ID3v2.3"
    }

    fn dissect(&self, file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
        dissect_id3v2_3_file(file)
    }

    fn can_handle(&self, header: &[u8]) -> bool {
        // Check for ID3v2.3 specifically
        if let Some((major, _minor)) = detect_id3v2_version(header) {
            return major == 3;
        }

        // Also check for MPEG sync (might contain ID3v2.3)
        detect_mpeg_sync(header)
    }

    fn name(&self) -> &'static str {
        "ID3v2.3 Dissector"
    }
}

/// Dissect an ID3v2.3 file from the beginning
pub fn dissect_id3v2_3_file(file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    // Read and parse ID3v2 header
    if let Some((major, minor, flags, size)) = read_id3v2_header(file)? {
        if major == 3 {
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
                if !flag_parts.is_empty() {
                    writeln!(&mut stdout, "Active: {}", flag_parts.join(", "))?;
                }
                stdout.reset()?;
            }

            writeln!(&mut stdout, "  Tag Size: {} bytes", size)?;

            if size > 100_000_000 {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
                writeln!(&mut stdout, "  WARNING: Extremely large tag size (> 100MB), verify file integrity")?;
                stdout.reset()?;
            } else if size > 50_000_000 {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))?;
                writeln!(&mut stdout, "  WARNING: Tag size is very large (> 50MB), likely rich podcast with chapter images")?;
                stdout.reset()?;
            } else if size > 10_000_000 {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
                writeln!(&mut stdout, "  INFO: Large tag size (> 10MB), possibly podcast with embedded chapter content")?;
                stdout.reset()?;
            }

            if size > 0 {
                // Allow very large tags for podcast content with chapter images
                dissect_id3v2_3(file, size, flags)?;
            }
        } else {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            writeln!(&mut stdout, "  Expected ID3v2.3, found version 2.{}", major)?;
            stdout.reset()?;
        }
    } else {
        writeln!(&mut stdout, "No ID3v2 header found")?;
    }

    Ok(())
}

pub fn dissect_id3v2_3(file: &mut File, tag_size: u32, flags: u8) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    // Diagnostic output
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    writeln!(&mut stdout, "\nDissecting ID3v2.3 tag (size: {} bytes, flags: 0x{:02X})...", tag_size, flags)?;
    stdout.reset()?;

    let mut buffer = vec![0u8; tag_size as usize];
    match file.read_exact(&mut buffer) {
        | Ok(_) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            writeln!(&mut stdout, "Successfully read {} bytes of tag data", tag_size)?;
            stdout.reset()?;
        }
        | Err(e) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
            writeln!(&mut stdout, "ERROR: Failed to read tag data: {}", e)?;
            stdout.reset()?;
            return Err(Box::new(e));
        }
    }

    // Handle unsynchronization if flag is set
    let unsync_flag = flags & 0x80 != 0; // Bit 7
    if unsync_flag {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        writeln!(&mut stdout, "  Unsynchronization detected - removing sync bytes")?;
        stdout.reset()?;
        buffer = remove_unsynchronization(&buffer);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        writeln!(&mut stdout, "  After unsynchronization removal: {} bytes", buffer.len())?;
        stdout.reset()?;
    }

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))?;
    writeln!(&mut stdout, "\nID3v2.3 Frames:")?;
    stdout.reset()?;

    // Check for extended header
    let mut frame_start = 0;
    if flags & 0x40 != 0 {
        // Extended header flag
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        writeln!(&mut stdout, "Extended header flag set, parsing...")?;
        stdout.reset()?;

        if buffer.len() >= 4 {
            // ID3v2.3 uses regular big-endian integer for extended header size
            let extended_size = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
            frame_start = 4 + extended_size as usize;

            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
            writeln!(&mut stdout, "  Extended header size: {} bytes", extended_size)?;
            writeln!(&mut stdout, "  Frame data starts at offset: {}", frame_start)?;
            stdout.reset()?;

            if frame_start > buffer.len() {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
                writeln!(&mut stdout, "  ERROR: Extended header size exceeds buffer length")?;
                stdout.reset()?;
                return Err("Invalid extended header size".into());
            }
        } else {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
            writeln!(&mut stdout, "  ERROR: Buffer too small to read extended header size")?;
            stdout.reset()?;
            return Err("Buffer too small for extended header".into());
        }
    }

    let mut pos = frame_start;
    let mut frame_count = 0;
    let mut parsing_errors = 0;
    let mut invalid_frames = 0;
    let mut chapter_count = 0;
    let mut image_count = 0;
    let mut large_frames = 0;
    let mut total_image_bytes = 0u64;

    while pos + 10 <= buffer.len() {
        // ID3v2.3 frame header: 4 bytes ID + 4 bytes size + 2 bytes flags
        let frame_id_bytes = &buffer[pos..pos + 4];
        let frame_id = std::str::from_utf8(frame_id_bytes).unwrap_or("????");

        // Stop if we hit padding (null bytes)
        if frame_id.starts_with('\0') {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
            writeln!(&mut stdout, "  Reached padding section at offset {}", pos)?;
            stdout.reset()?;
            break;
        }

        if frame_id.chars().all(|c| c.is_ascii_alphanumeric()) {
            // ID3v2.3 uses regular big-endian integers (not synchsafe)
            let frame_size = u32::from_be_bytes([buffer[pos + 4], buffer[pos + 5], buffer[pos + 6], buffer[pos + 7]]);
            let frame_flags = u16::from_be_bytes([buffer[pos + 8], buffer[pos + 9]]);

            // Diagnostic output for frame header
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
            writeln!(
                &mut stdout,
                "  Frame offset {}, ID bytes = [0x{:02X}, 0x{:02X}, 0x{:02X}, 0x{:02X}] (\"{}\"), Size bytes: [0x{:02X}, 0x{:02X}, 0x{:02X}, 0x{:02X}] = {} bytes, Flags: 0x{:04X}",
                pos,
                frame_id_bytes[0],
                frame_id_bytes[1],
                frame_id_bytes[2],
                frame_id_bytes[3],
                frame_id,
                buffer[pos + 4],
                buffer[pos + 5],
                buffer[pos + 6],
                buffer[pos + 7],
                frame_size,
                frame_flags
            )?;
            stdout.reset()?;

            // Validate frame ID for ID3v2.3
            if !is_valid_frame_for_version(frame_id, 3) {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                writeln!(&mut stdout, "    WARNING: Unknown frame ID for ID3v2.3: {}", frame_id)?;
                stdout.reset()?;
                invalid_frames += 1;
            }

            if frame_size > 0 && frame_size <= (buffer.len() - pos - 10) as u32 {
                // Track frame types for diagnostics
                if frame_id == "CHAP" {
                    chapter_count += 1;
                } else if frame_id == "APIC" {
                    image_count += 1;
                    total_image_bytes += frame_size as u64;
                }

                if frame_size > 1_000_000 {
                    large_frames += 1;
                }

                // Parse the frame using the new typed system
                match parse_id3v2_3_frame(&buffer, pos) {
                    | Some(frame) => {
                        frame_count += 1;
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                        write!(&mut stdout, "  {}", frame)?;
                        stdout.reset()?;
                    }
                    | None => {
                        parsing_errors += 1;
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
                        writeln!(&mut stdout, "    WARNING: Failed to parse frame, showing raw info")?;
                        stdout.reset()?;

                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                        write!(&mut stdout, "  Frame: {}", frame_id)?;
                        stdout.reset()?;
                        write!(&mut stdout, " (size: {} bytes", frame_size)?;

                        // Interpret frame flags for ID3v2.3
                        interpret_id3v2_3_frame_flags(frame_flags)?;
                        writeln!(&mut stdout, ")")?;
                    }
                }

                pos += 10 + frame_size as usize;
            } else if frame_size == 0 {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
                writeln!(&mut stdout, "    WARNING: Zero-sized frame, skipping")?;
                stdout.reset()?;
                pos += 10; // Skip frame header
            } else {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
                writeln!(&mut stdout, "    ERROR: Frame size {} exceeds remaining buffer ({} bytes)", frame_size, buffer.len() - pos - 10)?;
                stdout.reset()?;
                parsing_errors += 1;
                break;
            }
        } else {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
            writeln!(&mut stdout, "    Invalid frame ID format, stopping frame parsing")?;
            stdout.reset()?;
            break;
        }
    }

    // Summary diagnostics
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
    writeln!(&mut stdout, "\nID3v2.3 Parsing Summary:")?;
    stdout.reset()?;
    writeln!(&mut stdout, "  Frames parsed: {}", frame_count)?;
    writeln!(&mut stdout, "  Parsing errors: {}", parsing_errors)?;
    writeln!(&mut stdout, "  Invalid frame IDs: {}", invalid_frames)?;
    writeln!(&mut stdout, "  Bytes processed: {} / {}", pos, tag_size)?;

    // Enhanced statistics for large tags
    if chapter_count > 0 {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        writeln!(&mut stdout, "  Chapter frames (CHAP): {}", chapter_count)?;
        stdout.reset()?;
    }
    if image_count > 0 {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        writeln!(&mut stdout, "  Image frames (APIC): {} ({:.1} MB total)", image_count, total_image_bytes as f64 / 1_000_000.0)?;
        stdout.reset()?;
    }
    if large_frames > 0 {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        writeln!(&mut stdout, "  Large frames (>1MB): {}", large_frames)?;
        stdout.reset()?;
    }

    if pos < tag_size as usize {
        let remaining = tag_size as usize - pos;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        writeln!(&mut stdout, "  Unprocessed bytes: {} (likely padding)", remaining)?;
        stdout.reset()?;
    }

    Ok(())
}
