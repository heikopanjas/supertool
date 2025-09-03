use std::fs::File;

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
