use crate::id3v2_frame::Id3v2Frame;
use crate::id3v2_tools::*;
use crate::media_dissector::MediaDissector;
use std::fs::File;
use std::io::{Read, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// ID3v2.4 dissector for MP3 files
pub struct Id3v24Dissector;

/// Valid frame IDs for ID3v2.4 according to the specification
const VALID_ID3V2_4_FRAME_IDS: &[&str] = &[
    // Text information frames
    "TALB", "TBPM", "TCOM", "TCON", "TCOP", "TDEN", "TDLY", "TDOR", "TDRC", "TDRL", "TDTG", "TENC", "TEXT", "TFLT", "TIPL", "TIT1", "TIT2", "TIT3", "TKEY", "TLAN",
    "TLEN", "TMCL", "TMED", "TMOO", "TOAL", "TOFN", "TOLY", "TOPE", "TOWN", "TPE1", "TPE2", "TPE3", "TPE4", "TPOS", "TPRO", "TPUB", "TRCK", "TRSN", "TRSO", "TSOA",
    "TSOP", "TSOT", "TSRC", "TSSE", "TSST", "TXXX", // URL link frames
    "WCOM", "WCOP", "WOAF", "WOAR", "WOAS", "WORS", "WPAY", "WPUB", "WXXX", // Other frames
    "UFID", "MCDI", "ETCO", "MLLT", "SYTC", "USLT", "SYLT", "COMM", "RVA2", "EQU2", "RVRB", "PCNT", "POPM", "RBUF", "AENC", "LINK", "POSS", "USER", "OWNE", "COMR",
    "ENCR", "GRID", "PRIV", "GEOB", "APIC", "SEEK", "ASPI", "SIGN", // Chapter frames (ID3v2 Chapter Frame Addendum)
    "CHAP", "CTOC",
];

/// Parse an ID3v2.4 frame from raw buffer data
pub fn parse_id3v2_4_frame(buffer: &[u8], pos: usize) -> Option<Id3v2Frame> {
    if pos + 10 > buffer.len() {
        return None;
    }

    let frame_id = String::from_utf8_lossy(&buffer[pos..pos + 4]).to_string();

    // Stop if we hit padding (null bytes)
    if frame_id.starts_with('\0') || !frame_id.chars().all(|c| c.is_ascii_alphanumeric()) {
        return None;
    }

    // Check if this is a valid ID3v2.4 frame ID
    if !VALID_ID3V2_4_FRAME_IDS.contains(&frame_id.as_str()) {
        return None;
    }

    // ID3v2.4 uses synchsafe integers for frame size
    let frame_size = decode_synchsafe_int(&buffer[pos + 4..pos + 8]);
    let frame_flags = u16::from_be_bytes([buffer[pos + 8], buffer[pos + 9]]);

    if frame_size == 0 || frame_size > (buffer.len() - pos - 10) as u32 {
        return None;
    }

    let data = buffer[pos + 10..pos + 10 + frame_size as usize].to_vec();

    let mut frame = Id3v2Frame::new(frame_id, frame_size, frame_flags, data);

    // Parse the frame content using the new typed system (ID3v2.4)
    let _ = frame.parse_content(4); // Ignore parsing errors, keep raw data

    Some(frame)
}

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
                // Parse the frame using the new typed system
                if let Some(frame) = parse_id3v2_4_frame(&buffer, pos) {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                    write!(&mut stdout, "  {}", frame)?;
                    stdout.reset()?;
                } else {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                    write!(&mut stdout, "  Frame: {}", frame_id)?;
                    stdout.reset()?;
                    write!(&mut stdout, " (size: {} bytes", frame_size)?;

                    // Interpret frame flags for ID3v2.4
                    interpret_id3v2_4_frame_flags(frame_flags)?;
                    writeln!(&mut stdout, ")")?;
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
