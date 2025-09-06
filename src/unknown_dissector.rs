use crate::media_dissector::MediaDissector;
use std::fs::File;

/// Fallback dissector for unknown file formats
pub struct UnknownDissector;

impl MediaDissector for UnknownDissector {
    fn media_type(&self) -> &'static str {
        "Unknown"
    }

    fn dissect(&self, _file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
        println!("Unknown format - no suitable dissector available");
        Ok(())
    }

    fn can_handle(&self, _header: &[u8]) -> bool {
        true // Always can handle as fallback
    }

    fn name(&self) -> &'static str {
        "Unknown Format Dissector"
    }
}
