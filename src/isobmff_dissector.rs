use crate::dissector::MediaDissector;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// ISO Base Media File Format dissector for MP4 files
pub struct IsobmffDissector;

impl MediaDissector for IsobmffDissector {
    fn media_type(&self) -> &'static str {
        "ISO BMFF"
    }

    fn dissect(&self, file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
        dissect_isobmff(file)
    }

    fn can_handle(&self, header: &[u8]) -> bool {
        // ISO Base Media File Format detection - look for ftyp box
        if header.len() >= 8 && header[4..8] == [0x66, 0x74, 0x79, 0x70] {
            // "ftyp"
            return true;
        }
        false
    }

    fn name(&self) -> &'static str {
        "ISO BMFF Dissector"
    }
}

pub fn dissect_isobmff(file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    // Seek back to beginning
    file.seek(SeekFrom::Start(0))?;

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))?;
    writeln!(&mut stdout, "\nISO BMFF Boxes:")?;
    stdout.reset()?;

    let mut pos = 0u64;

    // Parse top-level boxes
    while pos < file.metadata()?.len() {
        file.seek(SeekFrom::Start(pos))?;

        let mut box_header = [0u8; 8];
        if file.read_exact(&mut box_header).is_err() {
            break;
        }

        let box_size = u32::from_be_bytes([box_header[0], box_header[1], box_header[2], box_header[3]]) as u64;
        let box_type = std::str::from_utf8(&box_header[4..8]).unwrap_or("????");

        if box_size < 8 {
            break;
        }

        writeln!(&mut stdout, "  Box: {} (size: {} bytes)", box_type, box_size)?;

        pos += box_size;

        // Prevent infinite loop
        if pos >= file.metadata()?.len() || box_size == 0 {
            break;
        }
    }

    Ok(())
}
