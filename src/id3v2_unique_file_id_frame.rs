/// Unique File Identifier Frame (UFID)
///
/// Structure: Owner identifier + Identifier

use crate::id3v2_text_encoding::decode_iso88591_string;

#[derive(Debug, Clone)]
pub struct UniqueFileIdFrame {
    pub owner_identifier: String,
    pub identifier: Vec<u8>,
}

impl UniqueFileIdFrame {
    /// Parse a UFID frame from raw data
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.is_empty() {
            return Err("UFID frame data is empty".to_string());
        }

        // Find null terminator for owner identifier
        let mut pos = 0;
        while pos < data.len() && data[pos] != 0 {
            pos += 1;
        }
        if pos >= data.len() {
            return Err("UFID owner identifier not null-terminated".to_string());
        }

        let owner_identifier = decode_iso88591_string(&data[0..pos]);
        pos += 1; // Skip null terminator

        // Identifier is the rest of the data (up to 64 bytes)
        let identifier = data[pos..].to_vec();
        if identifier.len() > 64 {
            return Err("UFID identifier too long (max 64 bytes)".to_string());
        }

        Ok(UniqueFileIdFrame {
            owner_identifier,
            identifier,
        })
    }
}
