use crate::cli::DebugOptions;
use std::fs::File;

/// Common trait for all media file dissectors
pub trait MediaDissector {
    /// The type of media this dissector handles
    fn media_type(&self) -> &'static str;

    /// Dissect the media file with specific output options
    fn dissect_with_options(&self, file: &mut File, options: &DebugOptions) -> Result<(), Box<dyn std::error::Error>>;

    /// Check if this dissector can handle the given file header
    fn can_handle(&self, header: &[u8]) -> bool;

    /// Get a descriptive name for this dissector
    fn name(&self) -> &'static str;
}
