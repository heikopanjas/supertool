use crate::id3v2_3_dissector::dissect_id3v2_3;
use crate::id3v2_4_dissector::dissect_id3v2_4;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn dissect_id3v2(file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    // Seek back to beginning
    file.seek(SeekFrom::Start(0))?;

    // Read first 10 bytes to check for ID3v2
    let mut id3_header = [0u8; 10];
    file.read_exact(&mut id3_header)?;

    if &id3_header[0..3] == b"ID3" {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))?;
        writeln!(&mut stdout, "\nID3v2 Header Found:")?;
        stdout.reset()?;

        let version_major = id3_header[3];
        let version_minor = id3_header[4];
        let flags = id3_header[5];

        // Add diagnostic output for raw header bytes
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
        writeln!(&mut stdout, "  Raw header bytes: {:02X?}", id3_header)?;
        stdout.reset()?;

        // Calculate tag size (synchsafe integer)
        let size = ((id3_header[6] as u32) << 21) | ((id3_header[7] as u32) << 14) | ((id3_header[8] as u32) << 7) | (id3_header[9] as u32);

        // Add diagnostic for size bytes and calculation
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        writeln!(&mut stdout, "  Size bytes: [0x{:02X}, 0x{:02X}, 0x{:02X}, 0x{:02X}]", id3_header[6], id3_header[7], id3_header[8], id3_header[9])?;
        writeln!(&mut stdout, "  Size calculation: ({} << 21) | ({} << 14) | ({} << 7) | {} = {}", id3_header[6], id3_header[7], id3_header[8], id3_header[9], size)?;
        stdout.reset()?;

        // Validate synchsafe format (each byte should have MSB = 0)
        let mut synchsafe_violation = false;
        for (i, &byte) in id3_header[6..10].iter().enumerate() {
            if byte & 0x80 != 0 {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
                writeln!(&mut stdout, "  WARNING: Size byte {} (0x{:02X}) violates synchsafe format (MSB set)!", i, byte)?;
                stdout.reset()?;
                synchsafe_violation = true;
            }
        }

        writeln!(&mut stdout, "  Version: 2.{}.{}", version_major, version_minor)?;
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

        // Add size validation warnings
        if size > 1_000_000 {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))?;
            writeln!(&mut stdout, "  WARNING: Tag size is unusually large (> 1MB)")?;
            stdout.reset()?;
        }

        if synchsafe_violation {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
            writeln!(&mut stdout, "  ERROR: Invalid synchsafe format detected in size field")?;
            stdout.reset()?;
        }

        if size > 0 && size < 1_000_000 {
            // Basic sanity check
            match version_major {
                | 3 => dissect_id3v2_3(file, size, flags)?,
                | 4 => dissect_id3v2_4(file, size, flags)?,
                | _ => {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                    writeln!(&mut stdout, "  Unsupported ID3v2 version: 2.{}", version_major)?;
                    stdout.reset()?;
                }
            }
        }
    } else {
        writeln!(&mut stdout, "No ID3v2 header found")?;
    }

    Ok(())
}
