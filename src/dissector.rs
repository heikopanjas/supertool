use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

/// Common trait for all media file dissectors
pub trait MediaDissector {
    /// The type of media this dissector handles
    fn media_type(&self) -> &'static str;

    /// Dissect the media file and output analysis results
    fn dissect(&self, file: &mut File) -> Result<(), Box<dyn std::error::Error>>;

    /// Check if this dissector can handle the given file header
    fn can_handle(&self, header: &[u8]) -> bool;

    /// Get a descriptive name for this dissector
    fn name(&self) -> &'static str;
}

/// Builder for creating the appropriate dissector based on file content
pub struct DissectorBuilder;

impl DissectorBuilder {
    /// Create a new dissector builder
    pub fn new() -> Self {
        Self
    }

    /// Analyze file header and return the appropriate dissector
    pub fn build_for_file(&self, file: &mut File) -> Result<Box<dyn MediaDissector>, Box<dyn std::error::Error>> {
        // Read file header for format detection
        let mut header = [0u8; 12];
        file.seek(SeekFrom::Start(0))?;
        file.read_exact(&mut header)?;
        file.seek(SeekFrom::Start(0))?; // Reset position

        // Try each dissector type in order of preference
        let dissectors: Vec<Box<dyn MediaDissector>> = vec![
            Box::new(crate::id3v2_3_dissector::Id3v23Dissector),
            Box::new(crate::id3v2_4_dissector::Id3v24Dissector),
            Box::new(crate::isobmff_dissector::IsobmffDissector),
        ];

        for dissector in dissectors {
            if dissector.can_handle(&header) {
                return Ok(dissector);
            }
        }

        // If no specific dissector found, return an unknown format dissector
        Ok(Box::new(UnknownDissector))
    }
}

impl Default for DissectorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Fallback dissector for unknown file formats
pub struct UnknownDissector;

impl MediaDissector for UnknownDissector {
    fn media_type(&self) -> &'static str {
        "Unknown"
    }

    fn dissect(&self, _file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Write;
        use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

        let mut stdout = StandardStream::stdout(ColorChoice::Auto);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        writeln!(&mut stdout, "Unknown format - no suitable dissector available")?;
        stdout.reset()?;

        Ok(())
    }

    fn can_handle(&self, _header: &[u8]) -> bool {
        true // Always can handle as fallback
    }

    fn name(&self) -> &'static str {
        "Unknown Format Dissector"
    }
}
