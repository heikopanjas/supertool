use crate::media_dissector::MediaDissector;
use crate::unknown_dissector::UnknownDissector;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

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
